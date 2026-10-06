//! Text, sound/speaker, and movie-clip datablocks — the odd corner of `bpy.data` a
//! model previously could only reach through `util_execute_python`.
//!
//! Three unrelated datablock families share this file because each is small and none
//! is the "primary" data of a scene the way a mesh or a material is:
//!
//! - `bpy.data.texts` — script/text buffers (driver expressions, OSL source, generated
//!   code). **Not** the 3D FONT curve object created by `text_create`/`text_update` in
//!   `mesh.rs` — that is a renderable object with a body; this is a headless text
//!   buffer with no transform. The `_datablock` suffix on every tool here exists only
//!   to keep the two apart in the tool list.
//! - `bpy.data.sounds` + `SPEAKER` objects — 3D positional audio. A speaker is created
//!   directly through `bpy.data.speakers.new()` / `bpy.data.objects.new()`, not
//!   `bpy.ops.object.speaker_add()`, so creation does not depend on `bpy.context`
//!   (active collection aside) or the current selection.
//! - `bpy.data.movieclips` — footage for camera/object tracking. Only load + list +
//!   basic settings are exposed; the tracking pipeline itself (markers, solving) is a
//!   separate, much deeper surface that is not worth adding speculatively.
//!
//! ## Ground truth, verified live against Blender 5.0 (`--background`)
//! - `Text.write()` **appends**; it does not clear first. `text_datablock_set` must
//!   call `clear()` before `write()` to actually overwrite, or the new body would be
//!   glued onto the old one.
//! - `bpy.data.sounds.load()` never raises — not even for a path that does not exist.
//!   A failed load (missing file, or a file Blender's audio backend cannot decode) is
//!   reported only by `sound.channels == "INVALID"` (a real load reports `MONO`,
//!   `STEREO`, …) and `samplerate == 0`. This is the audio-datablock analogue of
//!   `img_load`'s `has_data` check in `image.rs`, and it was found the same way: try
//!   it, don't assume it raises.
//! - `bpy.data.movieclips.load()` is a mixed bag: a genuinely missing path DOES raise
//!   `RuntimeError`, but an existing file with unreadable content does not — it hands
//!   back a clip with the placeholder size `(256, 256)` and, crucially,
//!   `frame_duration == 0`. A real single-frame source (even a still image loaded as a
//!   one-frame clip) reports `frame_duration == 1`, so `frame_duration == 0` is the
//!   reliable "nothing actually loaded" signal — verified against both a real PNG and
//!   a real 256x256 PNG (to rule out the placeholder size alone being a false
//!   positive: a genuinely valid 256x256 source reports `frame_duration == 1`, only a
//!   failed load reports `0`).
//! - `Speaker` (the SPEAKER object's `.data`) has settable `volume`, `pitch`,
//!   `attenuation`, `distance_max`, `distance_reference`, `cone_angle_outer`,
//!   `cone_angle_inner`, `cone_volume_outer`, `muted`, and `sound` (a pointer to a
//!   `bpy.data.sounds` entry, assigned by datablock, not by name string).
//! - `MovieClip.fps` is read-only (raises `AttributeError` on assignment); `size` is
//!   read-only. `frame_start`, `frame_offset`, and `colorspace_settings.name` are
//!   writable — that is the entire cheap, safe settings surface `movieclip_set` covers.

use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len, py_vec};

// ============================================================================
// Shared helpers
// ============================================================================

/// Look up `bpy.data.objects[name]`, requiring it to exist and be a SPEAKER.
fn get_speaker<'py>(objects: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    let obj_type: String = obj.getattr("type")?.extract()?;
    if obj_type != "SPEAKER" {
        return Err(PyValueError::new_err(format!(
            "`{name}` is a {obj_type} object, not a SPEAKER"
        )));
    }
    Ok(obj)
}

fn sound_info(s: &Bound<'_, PyAny>) -> PyResult<Value> {
    Ok(json!({
        "name": s.getattr("name")?.extract::<String>()?,
        "filepath": s.getattr("filepath")?.extract::<String>().unwrap_or_default(),
        "channels": s.getattr("channels")?.extract::<String>().unwrap_or_default(),
        "samplerate": s.getattr("samplerate")?.extract::<i64>().unwrap_or(0),
    }))
}

fn speaker_info(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    let spk = obj.getattr("data")?;
    let sound = spk.getattr("sound")?;
    let sound_name = if sound.is_none() {
        None
    } else {
        Some(sound.getattr("name")?.extract::<String>()?)
    };
    Ok(json!({
        "name": obj.getattr("name")?.extract::<String>()?,
        "location": py_vec(&obj.getattr("location")?)?,
        "sound": sound_name,
        "volume": spk.getattr("volume")?.extract::<f64>().unwrap_or(0.0),
        "pitch": spk.getattr("pitch")?.extract::<f64>().unwrap_or(0.0),
        "attenuation": spk.getattr("attenuation")?.extract::<f64>().unwrap_or(0.0),
        "distance_max": spk.getattr("distance_max")?.extract::<f64>().unwrap_or(0.0),
        "distance_reference": spk.getattr("distance_reference")?.extract::<f64>().unwrap_or(0.0),
        "cone_angle_outer": spk.getattr("cone_angle_outer")?.extract::<f64>().unwrap_or(0.0),
        "cone_angle_inner": spk.getattr("cone_angle_inner")?.extract::<f64>().unwrap_or(0.0),
        "cone_volume_outer": spk.getattr("cone_volume_outer")?.extract::<f64>().unwrap_or(0.0),
        "muted": spk.getattr("muted")?.extract::<bool>().unwrap_or(false),
    }))
}

fn movieclip_info(clip: &Bound<'_, PyAny>) -> PyResult<Value> {
    let size = clip.getattr("size")?;
    Ok(json!({
        "name": clip.getattr("name")?.extract::<String>()?,
        "filepath": clip.getattr("filepath")?.extract::<String>().unwrap_or_default(),
        "size": [
            size.get_item(0)?.extract::<i64>()?,
            size.get_item(1)?.extract::<i64>()?,
        ],
        "frame_duration": clip.getattr("frame_duration")?.extract::<i64>().unwrap_or(0),
        "frame_start": clip.getattr("frame_start")?.extract::<i64>().unwrap_or(1),
        "source": clip.getattr("source")?.extract::<String>().unwrap_or_default(),
    }))
}

// ============================================================================
// text_datablock_list / create / get / set / remove
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextDatablockList {}

crate::tool!(
    "text_datablock_list",
    "List every bpy.data.texts datablock (script/driver-expression buffers) with its line count. Not 3D text objects — see text_create for those.",
    TextDatablockList,
    |py: Python<'_>, _p: TextDatablockList| {
        let bpy = py.import("bpy")?;
        let texts = data(&bpy)?.getattr("texts")?;
        let mut out = Vec::new();
        for t in texts.try_iter()? {
            let t = t?;
            out.push(json!({
                "name": t.getattr("name")?.extract::<String>()?,
                "lines": py_len(py, &t.getattr("lines")?)?,
                "is_dirty": t.getattr("is_dirty")?.extract::<bool>().unwrap_or(false),
            }));
        }
        Ok(json!(out))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextDatablockCreate {
    /// Datablock name (a `bpy.data.texts` entry, not a 3D text object).
    pub name: String,
    /// Initial body. Omit for an empty buffer.
    pub body: Option<String>,
}

crate::tool!(
    "text_datablock_create",
    "Create a bpy.data.texts text datablock — a driver expression, OSL source, or a generated script for util_execute_python to run later. Not a 3D FONT object; see text_create for that.",
    TextDatablockCreate,
    |py: Python<'_>, p: TextDatablockCreate| {
        let bpy = py.import("bpy")?;
        let texts = data(&bpy)?.getattr("texts")?;
        let t = texts.call_method1("new", (p.name.as_str(),))?;
        if let Some(body) = &p.body {
            t.call_method1("write", (body.as_str(),))?;
        }
        Ok(json!({
            "name": t.getattr("name")?.extract::<String>()?,
            "body": t.call_method0("as_string")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextDatablockGet {
    /// Datablock name (see text_datablock_list).
    pub name: String,
}

crate::tool!(
    "text_datablock_get",
    "Read the full body of a bpy.data.texts datablock.",
    TextDatablockGet,
    |py: Python<'_>, p: TextDatablockGet| {
        let bpy = py.import("bpy")?;
        let texts = data(&bpy)?.getattr("texts")?;
        let t = texts.call_method1("get", (p.name.as_str(),))?;
        if t.is_none() {
            return Err(PyValueError::new_err(format!("text not found: {}", p.name)));
        }
        Ok(json!({
            "name": p.name,
            "body": t.call_method0("as_string")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextDatablockSet {
    /// Datablock name (see text_datablock_list).
    pub name: String,
    /// Replaces the entire body.
    pub body: String,
}

crate::tool!(
    "text_datablock_set",
    "Overwrite the body of a bpy.data.texts datablock (Text.write() appends, so this clears first).",
    TextDatablockSet,
    |py: Python<'_>, p: TextDatablockSet| {
        let bpy = py.import("bpy")?;
        let texts = data(&bpy)?.getattr("texts")?;
        let t = texts.call_method1("get", (p.name.as_str(),))?;
        if t.is_none() {
            return Err(PyValueError::new_err(format!("text not found: {}", p.name)));
        }
        t.call_method0("clear")?;
        t.call_method1("write", (p.body.as_str(),))?;
        Ok(json!({
            "name": p.name,
            "body": t.call_method0("as_string")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextDatablockRemove {
    /// Datablock name (see text_datablock_list).
    pub name: String,
}

crate::tool!(
    "text_datablock_remove",
    "Remove a bpy.data.texts datablock.",
    TextDatablockRemove,
    |py: Python<'_>, p: TextDatablockRemove| {
        let bpy = py.import("bpy")?;
        let texts = data(&bpy)?.getattr("texts")?;
        let t = texts.call_method1("get", (p.name.as_str(),))?;
        if t.is_none() {
            return Err(PyValueError::new_err(format!("text not found: {}", p.name)));
        }
        texts.call_method1("remove", (&t,))?;
        Ok(json!({ "removed": p.name }))
    }
);

// ============================================================================
// sound_load / sound_list
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SoundLoad {
    /// Absolute path to an audio file Blender can decode (wav, flac, mp3, ogg,
    /// ...). A path that loads no audio is an error, not a silent empty
    /// datablock.
    pub path: String,
    /// Name for the datablock. Default: the file name.
    pub name: Option<String>,
}

crate::tool!(
    "sound_load",
    "Load an audio file into bpy.data.sounds so a speaker_add can play it. Returns the datablock name, which speaker_add/speaker_set take as `sound`. For audio on the sequencer timeline instead of in 3D space, use vse_add_strip with type SOUND.",
    SoundLoad,
    |py: Python<'_>, p: SoundLoad| {
        let bpy = py.import("bpy")?;
        let sounds = data(&bpy)?.getattr("sounds")?;

        // `sounds.load()` never raises, even for a path that does not exist — a failed
        // decode is reported only through `channels` staying "INVALID" (a good load
        // reports MONO/STEREO/...) and `samplerate` staying 0. Verified live.
        let snd = sounds
            .call_method1("load", (p.path.as_str(),))
            .map_err(|e| PyIOError::new_err(format!("cannot load `{}`: {e}", p.path)))?;
        let channels: String = snd.getattr("channels")?.extract().unwrap_or_default();
        if channels == "INVALID" {
            sounds.call_method1("remove", (&snd,))?;
            return Err(PyIOError::new_err(format!(
                "`{}` loaded no audio data — is the path right, and is it an audio file Blender reads?",
                p.path
            )));
        }

        if let Some(name) = &p.name {
            snd.setattr("name", name.as_str())?;
        }

        sound_info(&snd)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SoundList {}

crate::tool!(
    "sound_list",
    "Every bpy.data.sounds audio datablock, with its filepath, channel layout and sample rate.",
    SoundList,
    |py: Python<'_>, _p: SoundList| {
        let bpy = py.import("bpy")?;
        let sounds = data(&bpy)?.getattr("sounds")?;
        let mut out = Vec::new();
        for s in sounds.try_iter()? {
            out.push(sound_info(&s?)?);
        }
        Ok(json!(out))
    }
);

// ============================================================================
// speaker_add / speaker_set / speaker_list
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SpeakerAdd {
    /// Object/datablock name. Blender appends .001 etc. on a clash, so read the
    /// returned `name` back. Default: "Speaker".
    pub name: Option<String>,
    /// World-space [x, y, z] in Blender units. Default: [0, 0, 0].
    pub location: Option<Vec<f64>>,
    /// Name of an already-loaded bpy.data.sounds datablock to assign (see
    /// sound_load). A name that does not exist is an error and nothing is
    /// created. Default: a silent speaker, assignable later with speaker_set.
    pub sound: Option<String>,
}

crate::tool!(
    "speaker_add",
    "Create a SPEAKER object for 3D positional audio. Optionally assigns an already-loaded sound (sound_load); tune playback with speaker_set.",
    SpeakerAdd,
    |py: Python<'_>, p: SpeakerAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let name = p.name.unwrap_or_else(|| "Speaker".to_string());

        // Resolved before anything is created, so a bad `sound` name cannot leave a
        // half-built speaker object behind.
        let sound_datablock = match &p.sound {
            Some(sound_name) => {
                let snd = d
                    .getattr("sounds")?
                    .call_method1("get", (sound_name.as_str(),))?;
                if snd.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "sound not found: {sound_name}"
                    )));
                }
                Some(snd)
            }
            None => None,
        };

        // Direct data-API creation (not `bpy.ops.object.speaker_add`) so this does not
        // depend on `bpy.context`'s selection/active state — only the active collection.
        let spk_data = d
            .getattr("speakers")?
            .call_method1("new", (name.as_str(),))?;
        let obj = d
            .getattr("objects")?
            .call_method1("new", (name.as_str(), &spk_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&obj,))?;

        let loc = p.location.unwrap_or_else(|| vec![0.0, 0.0, 0.0]);
        obj.setattr("location", PyTuple::new(py, loc)?)?;

        if let Some(snd) = sound_datablock {
            spk_data.setattr("sound", &snd)?;
        }

        speaker_info(&obj)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SpeakerSet {
    /// SPEAKER object name (see speaker_list).
    pub name: String,
    /// Playback gain, 0.0 (silent) to 1.0 (full). Blender clamps to that range.
    /// Speaker default: 1.0. Unchanged if omitted.
    pub volume: Option<f64>,
    /// Playback rate / pitch as a linear factor, clamped to 0.1-10.0: 1.0 is
    /// the original pitch, 2.0 one octave up. Speaker default: 1.0. Unchanged
    /// if omitted.
    pub pitch: Option<f64>,
    /// Rolloff exponent, >= 0: how quickly volume falls off with distance. 1.0
    /// (the default) is physically correct, 0.0 disables distance attenuation.
    pub attenuation: Option<f64>,
    /// Distance, in Blender units, beyond which volume no longer decreases.
    /// Default: effectively infinite (FLT_MAX).
    pub distance_max: Option<f64>,
    /// Distance, in Blender units, at which volume is exactly `volume` — the
    /// reference/rolloff point. Default: 1.0.
    pub distance_reference: Option<f64>,
    /// Outer cone angle in DEGREES, clamped to 0-360. Outside it the gain is
    /// `cone_volume_outer`. Default: 360 (omnidirectional).
    pub cone_angle_outer: Option<f64>,
    /// Inner cone angle in DEGREES, clamped to 0-360. Inside it the gain is
    /// full `volume`; between inner and outer it fades. Default: 360.
    pub cone_angle_inner: Option<f64>,
    /// Gain outside the outer cone, 0.0-1.0, as a fraction of `volume`.
    /// Default: 1.0 (no directional falloff).
    pub cone_volume_outer: Option<f64>,
    /// true silences this speaker without removing it. Unchanged if omitted.
    pub muted: Option<bool>,
    /// Name of a loaded bpy.data.sounds datablock to assign (see sound_load), or an
    /// empty string to unassign the current one.
    pub sound: Option<String>,
}

crate::tool!(
    "speaker_set",
    "Update a SPEAKER object's playback properties: volume, pitch, distance attenuation, cone angles, mute, and the assigned sound.",
    SpeakerSet,
    |py: Python<'_>, p: SpeakerSet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_speaker(&d.getattr("objects")?, &p.name)?;
        let spk = obj.getattr("data")?;

        if let Some(v) = p.volume {
            spk.setattr("volume", v)?;
        }
        if let Some(v) = p.pitch {
            spk.setattr("pitch", v)?;
        }
        if let Some(v) = p.attenuation {
            spk.setattr("attenuation", v)?;
        }
        if let Some(v) = p.distance_max {
            spk.setattr("distance_max", v)?;
        }
        if let Some(v) = p.distance_reference {
            spk.setattr("distance_reference", v)?;
        }
        if let Some(v) = p.cone_angle_outer {
            spk.setattr("cone_angle_outer", v)?;
        }
        if let Some(v) = p.cone_angle_inner {
            spk.setattr("cone_angle_inner", v)?;
        }
        if let Some(v) = p.cone_volume_outer {
            spk.setattr("cone_volume_outer", v)?;
        }
        if let Some(v) = p.muted {
            spk.setattr("muted", v)?;
        }
        if let Some(sound_name) = &p.sound {
            if sound_name.is_empty() {
                spk.setattr("sound", py.None())?;
            } else {
                let snd = d
                    .getattr("sounds")?
                    .call_method1("get", (sound_name.as_str(),))?;
                if snd.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "sound not found: {sound_name}"
                    )));
                }
                spk.setattr("sound", &snd)?;
            }
        }

        speaker_info(&obj)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SpeakerList {}

crate::tool!(
    "speaker_list",
    "Every SPEAKER object in the file, with its assigned sound and playback properties.",
    SpeakerList,
    |py: Python<'_>, _p: SpeakerList| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let mut out = Vec::new();
        for obj in objects.try_iter()? {
            let obj = obj?;
            if obj.getattr("type")?.extract::<String>()? == "SPEAKER" {
                out.push(speaker_info(&obj)?);
            }
        }
        Ok(json!(out))
    }
);

// ============================================================================
// movieclip_load / movieclip_list / movieclip_set
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MovieclipLoad {
    /// Absolute path to a movie file, or to the first frame of an image
    /// sequence. A file that yields no frames is an error, not a silent empty
    /// clip.
    pub path: String,
    /// Name for the datablock. Default: the file name.
    pub name: Option<String>,
}

crate::tool!(
    "movieclip_load",
    "Load footage (movie file or image sequence) into bpy.data.movieclips for camera/object tracking. Adjust it afterwards with movieclip_set; to put a video on the sequencer timeline instead, use vse_add_strip with type MOVIE.",
    MovieclipLoad,
    |py: Python<'_>, p: MovieclipLoad| {
        let bpy = py.import("bpy")?;
        let clips = data(&bpy)?.getattr("movieclips")?;

        // A genuinely missing path DOES raise here (unlike sounds.load). An existing
        // file Blender cannot decode does not raise, but leaves frame_duration at 0 —
        // a real source, even a single still frame, reports at least 1. Verified live
        // against a real PNG, a real 256x256 PNG (to rule out the placeholder-size
        // (256, 256) alone being a false positive) and unreadable/empty files.
        let clip = clips
            .call_method1("load", (p.path.as_str(),))
            .map_err(|e| PyIOError::new_err(format!("cannot load `{}`: {e}", p.path)))?;
        if clip
            .getattr("frame_duration")?
            .extract::<i64>()
            .unwrap_or(0)
            == 0
        {
            clips.call_method1("remove", (&clip,))?;
            return Err(PyIOError::new_err(format!(
                "`{}` loaded no frames — is the path right, and is it a video/image Blender reads?",
                p.path
            )));
        }

        if let Some(name) = &p.name {
            clip.setattr("name", name.as_str())?;
        }

        movieclip_info(&clip)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MovieclipList {}

crate::tool!(
    "movieclip_list",
    "Every bpy.data.movieclips datablock, with filepath, frame size and duration.",
    MovieclipList,
    |py: Python<'_>, _p: MovieclipList| {
        let bpy = py.import("bpy")?;
        let clips = data(&bpy)?.getattr("movieclips")?;
        let mut out = Vec::new();
        for c in clips.try_iter()? {
            out.push(movieclip_info(&c?)?);
        }
        Ok(json!(out))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MovieclipSet {
    /// Datablock name (see movieclip_list).
    pub name: String,
    /// Scene frame, as a whole number, on which the clip's first frame appears.
    /// Default: unchanged (1 for a freshly loaded clip).
    pub frame_start: Option<i64>,
    /// Whole-frame offset into the footage itself, i.e. how many source frames
    /// to skip. Default: unchanged.
    pub frame_offset: Option<i64>,
    /// OpenColorIO colour space name, exactly as this Blender's config spells it
    /// (verified on the stock 5.2 OCIO config: "sRGB", "Non-Color",
    /// "Linear Rec.709", "Linear Rec.2020", "ACEScg", "AgX Log", "Filmic Log",
    /// "Rec.1886", "Display P3", "scene_linear", and ~55 more camera log
    /// spaces). "Non-Color" is the one for tracking footage that must not get a
    /// gamma curve. An unknown name is an error.
    pub colorspace: Option<String>,
}

crate::tool!(
    "movieclip_set",
    "Update a loaded movie clip's frame mapping and colour space. (size and fps are read-only, set by the footage itself.)",
    MovieclipSet,
    |py: Python<'_>, p: MovieclipSet| {
        let bpy = py.import("bpy")?;
        let clips = data(&bpy)?.getattr("movieclips")?;
        let clip = clips.call_method1("get", (p.name.as_str(),))?;
        if clip.is_none() {
            return Err(PyValueError::new_err(format!(
                "movie clip not found: {}",
                p.name
            )));
        }

        if let Some(v) = p.frame_start {
            clip.setattr("frame_start", v)?;
        }
        if let Some(v) = p.frame_offset {
            clip.setattr("frame_offset", v)?;
        }
        if let Some(cs) = &p.colorspace {
            clip.getattr("colorspace_settings")?
                .setattr("name", cs.as_str())
                .map_err(|e| PyValueError::new_err(format!("unknown colour space `{cs}`: {e}")))?;
        }

        movieclip_info(&clip)
    }
);
