//! Small helpers over the `bpy` Python API. Shared by every tool module.

use pyo3::prelude::*;
use pyo3::types::PyAny;

/// `len(x)` in Python space — works for `bpy.data.objects` and friends, which are
/// sized collections but not Rust iterators.
pub fn py_len(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<usize> {
    py.import("builtins")?
        .getattr("len")?
        .call1((obj,))?
        .extract()
}

pub fn scene<'py>(bpy: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    bpy.getattr("context")?.getattr("scene")
}

pub fn data<'py>(bpy: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    bpy.getattr("data")
}

/// Run a `bpy.ops` result through a check that it actually did something.
///
/// **An operator that declines to act does not raise.** `bpy.ops` returns a set:
/// `{'FINISHED'}` when it worked, `{'CANCELLED'}` when its poll passed but it changed
/// nothing — `transform_apply` with nothing selected, `join` with one object,
/// `remove_doubles` with no selection. Ignoring the return value means reporting success
/// for an operation that never happened, which is the exact "failure disguised as
/// success" this codebase forbids, arriving from Blender instead of from us.
///
/// Pass the operator's return value and a description of what was attempted.
pub fn op_ok(result: &Bound<'_, PyAny>, what: &str) -> PyResult<()> {
    let mut flags = Vec::new();
    for f in result.try_iter()? {
        flags.push(f?.extract::<String>()?);
    }
    if flags.iter().any(|f| f == "FINISHED") {
        return Ok(());
    }
    Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
        "{what} did nothing ({}). Blender declined the operation — usually the wrong \
         selection, mode, or nothing to act on.",
        if flags.is_empty() {
            "no result".to_string()
        } else {
            flags.join(", ")
        }
    )))
}

/// Write a JSON value onto an RNA property by name.
///
/// Blender's settings are wildly type-specific — a constraint, a modifier and a node all
/// carry different fields — so anything generic has to go through `setattr` and let RNA
/// decide. A bad enum value is answered with the values this Blender accepts, because
/// that is the second place a model's memory goes stale (right after socket names).
pub fn set_rna(obj: &Bound<'_, PyAny>, key: &str, value: &serde_json::Value) -> PyResult<()> {
    use serde_json::Value;

    let py = obj.py();
    let result = match value {
        Value::Bool(b) => obj.setattr(key, *b),
        Value::String(s) => obj.setattr(key, s.as_str()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                obj.setattr(key, i)
            } else {
                obj.setattr(
                    key,
                    n.as_f64()
                        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("not a number"))?,
                )
            }
        }
        Value::Array(a) => {
            let nums: Vec<f64> = a.iter().filter_map(Value::as_f64).collect();
            if nums.len() != a.len() {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "`{key}` takes numbers only"
                )));
            }
            obj.setattr(key, pyo3::types::PyTuple::new(py, nums)?)
        }
        Value::Null | Value::Object(_) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "`{key}` cannot take {value}"
            )));
        }
    };

    result.map_err(|e| match enum_values(obj, key) {
        Ok(Some(vals)) => pyo3::exceptions::PyValueError::new_err(format!(
            "`{key}` does not accept {value}. Allowed: {}",
            vals.join(", ")
        )),
        _ => pyo3::exceptions::PyRuntimeError::new_err(format!("setting `{key}` failed: {e}")),
    })
}

/// The values an enum property accepts. `None` when it is not an enum.
///
/// ## Why this is not just `bl_rna.enum_items`
/// Some of Blender's enums are **dynamic** (`itemf`): their items are produced by a
/// callback that needs the instance, and reading them off the class's `bl_rna` returns
/// an incomplete list. `render.engine` is the trap: `enum_items` reports only
/// `BLENDER_EEVEE`, while `CYCLES` and `BLENDER_WORKBENCH` are perfectly settable. Code
/// that trusted `enum_items` therefore *rejected valid engines* — that was a real bug
/// here, and it rejected Cycles for every render.
///
/// Blender's own error message is the reliable source: assigning a value no item can
/// equal raises with the live tuple in the text. The assignment fails before mutating
/// anything, so this is a read, not a write.
pub fn enum_values(obj: &Bound<'_, PyAny>, key: &str) -> PyResult<Option<Vec<String>>> {
    let prop = obj
        .getattr("bl_rna")?
        .getattr("properties")?
        .call_method1("get", (key,))?;
    if prop.is_none() || prop.getattr("type")?.extract::<String>()? != "ENUM" {
        return Ok(None);
    }

    // Ask Blender, by making it complain.
    if let Err(e) = obj.setattr(key, "\u{1}__mcp_enum_probe__") {
        let msg = e.to_string();
        if let Some(open) = msg.find("not found in (")
            && let Some(close) = msg[open..].find(')')
        {
            let inner = &msg[open + "not found in (".len()..open + close];
            let items: Vec<String> = inner
                .split(',')
                .map(|s| s.trim().trim_matches(&['\'', '"'][..]).to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !items.is_empty() {
                return Ok(Some(items));
            }
        }
    }

    // Static enum: the class metadata is complete and correct.
    let mut out = Vec::new();
    for item in prop.getattr("enum_items")?.try_iter()? {
        out.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    Ok(Some(out))
}

/// Read a Python float sequence (a `Vector`, a colour, `location`…) into a `Vec`.
pub fn py_vec(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let mut v = Vec::new();
    for x in seq.try_iter()? {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

/// Resolve a render-engine name against the engines this Blender actually has.
///
/// The engine id is a moving target: EEVEE was `BLENDER_EEVEE`, became
/// `BLENDER_EEVEE_NEXT` in 4.2, and is `BLENDER_EEVEE` again in 5.0. Code that
/// hardcoded `BLENDER_EEVEE_NEXT` simply raised on Blender 5. So ask the enum, accept
/// the friendly alias a caller will type, and if it still does not match, say what
/// this Blender does offer instead of failing with Blender's opaque message.
pub fn render_engine(scene: &Bound<'_, PyAny>, want: &str) -> PyResult<String> {
    let render = scene.getattr("render")?;

    // `engine` is a DYNAMIC enum: reading it off `bl_rna.enum_items` returns only
    // BLENDER_EEVEE and hides Cycles and Workbench, which are perfectly settable. Going
    // through `enum_values` — which asks Blender itself — is what makes CYCLES work.
    let available = enum_values(&render, "engine")?.ok_or_else(|| {
        pyo3::exceptions::PyRuntimeError::new_err("render.engine is not an enum?")
    })?;

    let want = want.to_uppercase();
    if let Some(exact) = available.iter().find(|e| **e == want) {
        return Ok(exact.clone());
    }
    // "EEVEE" -> whatever this build calls EEVEE; likewise CYCLES, WORKBENCH.
    if let Some(alias) = available.iter().find(|e| e.contains(&want)) {
        return Ok(alias.clone());
    }

    Err(pyo3::exceptions::PyValueError::new_err(format!(
        "unknown render engine `{want}`. This Blender has: {}",
        available.join(", ")
    )))
}

/// Every F-Curve of an Action, across Blender's two Action layouts.
///
/// Blender 4.4 introduced slotted Actions and 5.0 **removed `Action.fcurves`
/// outright** — curves now live in a channelbag reached through
/// `action.layers[].strips[].channelbag(slot)`. Code written against the old flat
/// attribute raises `AttributeError` on 5.0, so every reader must go through here.
pub fn action_fcurves<'py>(action: &Bound<'py, PyAny>) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let mut out = Vec::new();

    // Blender >= 4.4: layered/slotted.
    if let Ok(layers) = action.getattr("layers") {
        let slots = action.getattr("slots")?;
        for layer in layers.try_iter()? {
            for strip in layer?.getattr("strips")?.try_iter()? {
                let strip = strip?;
                for slot in slots.try_iter()? {
                    // A strip has no channelbag for a slot it does not animate.
                    let Ok(bag) = strip.call_method1("channelbag", (slot?,)) else {
                        continue;
                    };
                    if bag.is_none() {
                        continue;
                    }
                    for fc in bag.getattr("fcurves")?.try_iter()? {
                        out.push(fc?);
                    }
                }
            }
        }
        return Ok(out);
    }

    // Blender <= 4.3: flat list.
    for fc in action.getattr("fcurves")?.try_iter()? {
        out.push(fc?);
    }
    Ok(out)
}

/// Where a Geometry Nodes modifier keeps one input socket's value.
///
/// Blender 5.2 moved these off the modifier's ID-properties
/// (`modifier["Socket_2"]`, which now raises "id properties not supported for this
/// type") onto a typed interface: `modifier.properties.inputs.Socket_2`, whose
/// `.value` holds the value, `.type` says VALUE vs ATTRIBUTE, and `.attribute_name`
/// names the attribute in the ATTRIBUTE case. Verified live on 5.2.2.
///
/// Blender 5.0/5.1 have no `.properties`, so this returns `None` there and the
/// caller falls back to the ID-property address rather than failing.
pub fn modifier_input_slot<'py>(
    modifier: &Bound<'py, PyAny>,
    identifier: &str,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    if !modifier.hasattr("properties")? {
        return Ok(None);
    }
    let inputs = modifier.getattr("properties")?.getattr("inputs")?;
    if !inputs.hasattr(identifier)? {
        return Ok(None);
    }
    Ok(Some(inputs.getattr(identifier)?))
}

/// Write one value into a modifier input, at whichever address this Blender has.
pub fn write_modifier_input<'py, V>(
    modifier: &Bound<'py, PyAny>,
    identifier: &str,
    value: V,
) -> PyResult<()>
where
    V: pyo3::IntoPyObject<'py>,
{
    match modifier_input_slot(modifier, identifier)? {
        Some(slot) => slot.setattr("value", value),
        None => modifier.set_item(identifier, value),
    }
}

/// Read one modifier input's value back, at whichever address this Blender has.
pub fn read_modifier_input<'py>(
    modifier: &Bound<'py, PyAny>,
    identifier: &str,
) -> PyResult<Bound<'py, PyAny>> {
    match modifier_input_slot(modifier, identifier)? {
        Some(slot) => slot.getattr("value"),
        None => modifier.get_item(identifier),
    }
}
