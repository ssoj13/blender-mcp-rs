//! `kdb` — the knowledge base the server ships, and the tools that search it.
//!
//! ## Why this exists
//! `tools/list` tells a model *what* it can call, never *how Blender works*. An agent
//! that has never seen this Blender guesses enum values, socket names and workflows —
//! the exact failure mode this project keeps paying for. So the server carries a
//! corpus and lets the model search it: `kdb_search` to find, `kdb_read` to read,
//! `kdb_guide` for the short per-area briefings.
//!
//! ## A third class of tool
//! Scene tools run inside Blender; control tools run in the server process and act on
//! the OS process (`control.rs`). These are the third kind: they answer **without
//! Blender running at all**, which is the point — an agent should be able to read how
//! to do something before it launches anything. That also keeps the corpus out of the
//! `.pyd`, which would otherwise carry 7 MB it can never use.
//!
//! ## Where the corpus comes from
//! `python/kdb_build.py` regenerates `kdb/`: the API reference is dumped from a live
//! Blender (never scraped, never guessed), the release notes and manual are pinned to
//! the targeted version, and `kdb/guides/` is hand-written. Everything is Markdown with
//! a small frontmatter block (`area`, `source`, `version`).
//!
//! ## Search
//! BM25 over the whole corpus — lexical, in-process, no model and no FFI (see the
//! no-FFI rule). A model writes good keyword queries, and the corpus is technical text
//! where the exact token (`shade_auto_smooth`, `NodeSocketFloat`) is what matters, so
//! lexical retrieval is the right tool rather than a compromise. Hand-written guides
//! outrank generated reference at equal relevance, since a guide answers "how do I",
//! which is usually the real question.

use std::collections::HashMap;
use std::sync::OnceLock;

use include_dir::{Dir, include_dir};
use serde_json::{Value, json};

/// The corpus, baked into the binary at build time.
static EMBEDDED: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/kdb");

/// Extra documents a user can drop in at runtime: `$BLENDER_MCP_KDB` may point at a
/// directory of `.md` files (any nesting). They join the same index and are marked
/// `local`, so a studio can add its own conventions without rebuilding anything.
const OVERLAY_ENV: &str = "BLENDER_MCP_KDB";

/// BM25 constants. Standard defaults: `k1` bounds term-frequency saturation, `b`
/// controls length normalisation.
const K1: f64 = 1.2;
const B: f64 = 0.75;

/// How much a document's own kind is worth, all else equal. A guide is written for
/// this server's tools; the manual describes the GUI and is the last resort.
fn area_weight(area: &str) -> f64 {
    match area {
        a if a.starts_with("guide") => 1.6,
        a if a.starts_with("api") => 1.25,
        "release-notes" => 1.1,
        _ => 1.0,
    }
}

pub struct Doc {
    pub path: String,
    pub area: String,
    /// Where the text came from (`blender-manual`, `blender-5.2`, `local`, …). Part of
    /// every hit so a reader can weigh generated reference against a written guide.
    pub source: String,
    pub version: String,
    pub title: String,
    pub body: String,
    /// term -> count, for the body and title together.
    terms: HashMap<String, u32>,
    len: f64,
}

pub struct Index {
    docs: Vec<Doc>,
    /// term -> number of documents containing it.
    df: HashMap<String, u32>,
    avg_len: f64,
}

/// Split into lowercase tokens, keeping the characters that carry meaning in API
/// names: `bpy.ops.object.shade_auto_smooth` must stay findable as a whole and by its
/// parts, so it is emitted both ways.
fn tokenize(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.')) {
        let t = raw.trim_matches('.').to_lowercase();
        if t.is_empty() || t.len() > 64 {
            continue;
        }
        if t.contains('.') || t.contains('_') {
            for part in t.split(['.', '_']) {
                if part.len() > 1 {
                    out.push(part.to_string());
                }
            }
        }
        out.push(t);
    }
    out
}

/// Pull the `---` frontmatter off a document, returning (fields, body).
///
/// CRLF is tolerated: the shipped corpus is LF, but an overlay document written on
/// Windows would otherwise fail to parse *silently* — every field lost, the area
/// guessed from the path, and nothing to see in the output.
fn split_frontmatter(text: &str) -> (HashMap<String, String>, &str) {
    let mut fields = HashMap::new();
    let Some(rest) = text
        .strip_prefix("---\r\n")
        .or_else(|| text.strip_prefix("---\n"))
    else {
        return (fields, text);
    };
    let Some((end, skip)) = rest
        .find("\r\n---\r\n")
        .map(|i| (i, 7))
        .or_else(|| rest.find("\n---\n").map(|i| (i, 5)))
    else {
        return (fields, text);
    };
    for line in rest[..end].lines() {
        if let Some((k, v)) = line.split_once(':') {
            fields.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    (fields, rest[end + skip..].trim_start_matches(['\r', '\n']))
}

/// First Markdown heading, else the file stem — what a search hit is labelled with.
fn title_of(body: &str, path: &str) -> String {
    for line in body.lines().take(40) {
        if let Some(h) = line.trim().strip_prefix("# ") {
            return h.trim().to_string();
        }
    }
    path.rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".md")
        .replace(['-', '_'], " ")
}

fn make_doc(path: String, text: &str, default_source: &str) -> Doc {
    let (fields, body) = split_frontmatter(text);
    let area = fields
        .get("area")
        .cloned()
        .unwrap_or_else(|| path.split('/').next().unwrap_or("misc").to_string());
    let title = title_of(body, &path);
    let mut terms: HashMap<String, u32> = HashMap::new();
    let mut len = 0.0;
    for t in tokenize(&title).into_iter().chain(tokenize(body)) {
        *terms.entry(t).or_insert(0) += 1;
        len += 1.0;
    }
    Doc {
        path,
        area,
        source: fields
            .get("source")
            .cloned()
            .unwrap_or_else(|| default_source.to_string()),
        version: fields.get("version").cloned().unwrap_or_default(),
        title,
        body: body.to_string(),
        terms,
        len,
    }
}

fn collect(dir: &Dir<'_>, docs: &mut Vec<Doc>) {
    for file in dir.files() {
        let path = file.path().to_string_lossy().replace('\\', "/");
        if !path.ends_with(".md") {
            continue;
        }
        if let Some(text) = file.contents_utf8() {
            docs.push(make_doc(path, text, "blender"));
        }
    }
    for sub in dir.dirs() {
        collect(sub, docs);
    }
}

fn collect_overlay(root: &std::path::Path, docs: &mut Vec<Doc>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_overlay(&path, docs);
        } else if path.extension().is_some_and(|e| e == "md")
            && let Ok(text) = std::fs::read_to_string(&path)
        {
            let rel = path.to_string_lossy().replace('\\', "/");
            docs.push(make_doc(format!("local:{rel}"), &text, "local"));
        }
    }
}

pub fn index() -> &'static Index {
    static INDEX: OnceLock<Index> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut docs = Vec::new();
        collect(&EMBEDDED, &mut docs);
        if let Ok(dir) = std::env::var(OVERLAY_ENV) {
            collect_overlay(std::path::Path::new(&dir), &mut docs);
        }
        let mut df: HashMap<String, u32> = HashMap::new();
        for d in &docs {
            for t in d.terms.keys() {
                *df.entry(t.clone()).or_insert(0) += 1;
            }
        }
        let avg_len = if docs.is_empty() {
            1.0
        } else {
            docs.iter().map(|d| d.len).sum::<f64>() / docs.len() as f64
        };
        Index { docs, df, avg_len }
    })
}

impl Index {
    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn get(&self, path: &str) -> Option<&Doc> {
        self.docs
            .iter()
            .find(|d| d.path == path || d.path.trim_end_matches(".md") == path)
    }

    /// BM25, with the per-area weight applied to the final score.
    pub fn search(&self, query: &str, area: Option<&str>, limit: usize) -> Vec<(&Doc, f64)> {
        let q = tokenize(query);
        let n = self.docs.len() as f64;
        let mut scored: Vec<(&Doc, f64)> = self
            .docs
            .iter()
            .filter(|d| area.is_none_or(|a| d.area.contains(a)))
            .filter_map(|d| {
                let mut score = 0.0;
                for term in &q {
                    let Some(&tf) = d.terms.get(term) else {
                        continue;
                    };
                    let df = *self.df.get(term).unwrap_or(&1) as f64;
                    let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();
                    let tf = tf as f64;
                    score +=
                        idf * (tf * (K1 + 1.0)) / (tf + K1 * (1.0 - B + B * d.len / self.avg_len));
                }
                (score > 0.0).then(|| (d, score * area_weight(&d.area)))
            })
            .collect();
        scored.sort_by(|a, b| b.1.total_cmp(&a.1));
        scored.truncate(limit);
        scored
    }
}

/// A few lines around the first query term, so a hit is judgeable without a second call.
fn snippet(doc: &Doc, query: &str) -> String {
    let q: Vec<String> = tokenize(query);
    let lines: Vec<&str> = doc.body.lines().collect();
    let hit = lines.iter().position(|line| {
        let lower = line.to_lowercase();
        q.iter().any(|t| lower.contains(t.as_str()))
    });
    let start = hit.map(|h| h.saturating_sub(1)).unwrap_or(0);
    let text: String = lines
        .iter()
        .skip(start)
        .take(6)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let text = text.trim().to_string();
    if text.chars().count() > 600 {
        text.chars().take(600).collect::<String>() + " …"
    } else {
        text
    }
}

// ---------------------------------------------------------------------------
// tools
// ---------------------------------------------------------------------------

pub fn kdb_tools() -> Vec<(&'static str, String, Value)> {
    let n = index().len();
    vec![
        (
            "kdb_search",
            format!(
                "Search the built-in Blender knowledge base ({n} documents: hand-written \
                 guides, the API reference dumped from this exact Blender, release notes, \
                 and the user manual). Use it BEFORE guessing an enum value, a node socket \
                 name, or how a workflow is done — and when a tool call fails in a way that \
                 smells like a version difference. Returns ranked hits with a snippet and a \
                 `path` for kdb_read. Works with Blender not running."
            ),
            json!({"type": "object", "properties": {
                "query": {"type": "string",
                    "description": "Keywords, not a sentence. API names work well verbatim, e.g. \"shade_auto_smooth angle\" or \"geometry nodes modifier input attribute\"."},
                "area": {"type": "string",
                    "description": "Optional filter on the document area, matched as a substring: guide, api-types, api-ops, api-nodes, release-notes, manual-modeling, manual-render, …"},
                "limit": {"type": "integer",
                    "description": "How many hits to return (default 8, max 25)."}
            }, "required": ["query"]}),
        ),
        (
            "kdb_read",
            "Read one knowledge-base document whole, by the `path` a kdb_search hit \
             reports. Long documents are paged: pass `offset` to continue."
                .to_string(),
            json!({"type": "object", "properties": {
                "path": {"type": "string", "description": "Document path from a kdb_search hit, e.g. \"api/nodes/geometry.md\"."},
                "offset": {"type": "integer", "description": "Line to start from (default 0)."},
                "limit": {"type": "integer", "description": "How many lines to return (default 400)."}
            }, "required": ["path"]}),
        ),
        (
            "kdb_guide",
            "The short per-area briefings: what the area is, which tools of this server \
             do what in it, and the traps. Call with no argument to list the guides, or \
             with `area` to read one. Start here when beginning work in an unfamiliar \
             area, rather than probing tools one at a time."
                .to_string(),
            json!({"type": "object", "properties": {
                "area": {"type": "string", "description": "Guide name, e.g. \"modelling\", \"shading\", \"geometry-nodes\", \"animation\", \"rigging\", \"physics\", \"rendering\"."}
            }}),
        ),
    ]
}

pub fn dispatch(name: &str, args: &Value) -> Option<Result<String, String>> {
    let result = match name {
        "kdb_search" => search_tool(args),
        "kdb_read" => read_tool(args),
        "kdb_guide" => guide_tool(args),
        _ => return None,
    };
    Some(result.map(|v| v.to_string()))
}

fn search_tool(args: &Value) -> Result<Value, String> {
    let query = args
        .get("query")
        .and_then(Value::as_str)
        .ok_or("kdb_search needs a `query`")?;
    let area = args.get("area").and_then(Value::as_str);
    let limit = args
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(8)
        .clamp(1, 25) as usize;

    let idx = index();
    let hits: Vec<Value> = idx
        .search(query, area, limit)
        .into_iter()
        .map(|(d, score)| {
            json!({
                "path": d.path,
                "title": d.title,
                "area": d.area,
                "source": d.source,
                "version": d.version,
                "score": (score * 100.0).round() / 100.0,
                "snippet": snippet(d, query),
            })
        })
        .collect();

    if hits.is_empty() {
        return Ok(json!({
            "query": query,
            "hits": [],
            "hint": "Nothing matched. Try fewer, more literal keywords (an API name, an \
                     enum value, a node label), or drop the `area` filter.",
        }));
    }
    Ok(json!({ "query": query, "hits": hits }))
}

fn read_tool(args: &Value) -> Result<Value, String> {
    let path = args
        .get("path")
        .and_then(Value::as_str)
        .ok_or("kdb_read needs a `path`")?;
    let offset = args.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = args
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(400)
        .clamp(1, 2000) as usize;

    let idx = index();
    let doc = idx.get(path).ok_or_else(|| {
        format!("no document `{path}` in the knowledge base — use kdb_search to find one")
    })?;
    let lines: Vec<&str> = doc.body.lines().collect();
    let end = (offset + limit).min(lines.len());
    let slice = lines.get(offset..end).unwrap_or(&[]).join("\n");
    Ok(json!({
        "path": doc.path,
        "title": doc.title,
        "area": doc.area,
        "version": doc.version,
        "lines": lines.len(),
        "offset": offset,
        "returned": end.saturating_sub(offset),
        "more": end < lines.len(),
        "text": slice,
    }))
}

fn guide_tool(args: &Value) -> Result<Value, String> {
    let idx = index();
    let guides: Vec<&Doc> = idx
        .docs
        .iter()
        .filter(|d| d.area.starts_with("guide") || d.path.starts_with("guides/"))
        .collect();

    match args.get("area").and_then(Value::as_str) {
        None => Ok(json!({
            "guides": guides.iter().map(|d| json!({
                "area": d.path.trim_start_matches("guides/").trim_end_matches(".md"),
                "title": d.title,
                "path": d.path,
            })).collect::<Vec<_>>(),
        })),
        Some(want) => {
            let key = want.to_lowercase();
            let doc = guides
                .iter()
                .find(|d| {
                    d.path.trim_start_matches("guides/").trim_end_matches(".md") == key
                        || d.title.to_lowercase() == key
                })
                .ok_or_else(|| {
                    let have: Vec<&str> = guides
                        .iter()
                        .map(|d| d.path.trim_start_matches("guides/").trim_end_matches(".md"))
                        .collect();
                    format!("no guide `{want}`. Available: {}", have.join(", "))
                })?;
            Ok(json!({
                "area": key,
                "title": doc.title,
                "path": doc.path,
                "text": doc.body,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_is_embedded_and_indexed() {
        let idx = index();
        assert!(
            idx.len() > 100,
            "the knowledge base looks empty ({} docs) — did `python/kdb_build.py` run?",
            idx.len()
        );
    }

    #[test]
    fn search_finds_an_api_name_verbatim() {
        let hits = index().search("shade_auto_smooth", None, 5);
        assert!(!hits.is_empty(), "no hit for a literal operator name");
    }

    #[test]
    fn every_document_has_a_title() {
        for d in &index().docs {
            assert!(!d.title.trim().is_empty(), "{} has no title", d.path);
        }
    }
}
