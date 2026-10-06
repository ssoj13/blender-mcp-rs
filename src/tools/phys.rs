//! Physics, particle, force-field, geometry-nodes-simulation and render-queue
//! tools. Registered automatically — nothing outside this file names them.
//!
//! Ported from `addon/mcp_server/__init__.py` (`_h_phys_*`, `_h_sim_*`,
//! `_h_render_queue_*`). A handful of bugs in the original were fixed rather
//! than carried over verbatim; each fix is documented at its call site:
//!
//! - `render_queue_add`: job ids are now a monotonic counter instead of
//!   `len(queue) + 1`, which reissued a deleted job's id.
//! - `render_queue_clear`: `completed_only` now drops exactly the completed
//!   jobs instead of also discarding failed ones.
//! - `sim_bake` and `phys_bake`: `frame_start`/`frame_end` now genuinely drive the
//!   bake (through the scene frame range, which is what the bake operators read)
//!   instead of being echoed back in the result while the bake used a different
//!   range. The scene is restored afterwards either way.
//! - `phys_update_particles`: the `ps_name` fallback is now evaluated lazily,
//!   so supplying `ps_name` no longer crashes on an object with zero particle
//!   systems.
//! - `phys_add`: an unknown physics type used to add nothing and report success.
//!   It is now an error.
//! - `sim_create_zone`: verified live that a fresh `GeometryNodeSimulationInput`
//!   carries no Geometry socket at all until `pair_with_output()` is called — before
//!   that, `sim_in.paired_output` is `None`, `sim_in.inputs` is empty, and
//!   `sim_in.outputs` is only `Delta Time`. The original code never paired the two
//!   nodes and linked `outputs[0]` to `inputs[0]` regardless — Delta Time into Skip,
//!   a float into a boolean socket. `tree.links.new` did not reject it, so the tool
//!   reported success while producing two nodes that were never actually a
//!   simulation zone. Fixed to pair first, then link the Geometry sockets by name.
//!
//! Parameters the original advertised and then ignored (`sim_set_params`'s
//! `use_cache` and `node_group`, `render_queue_start`'s `background`) are no longer
//! advertised. A parameter the model can set and we silently drop is a lie the
//! schema tells on our behalf; implementing them for real would be a new feature,
//! not a port.
//!
//! Rigid body, soft body, cloth, collision, fluid-modifier, particle-system and
//! force-field RNA are all unchanged from Blender 4.x. The geometry-nodes
//! simulation zone node types themselves (`GeometryNodeSimulationInput`/`Output`)
//! are stable since their 4.0 introduction — the breakage was in how this file
//! wired them, not in Blender's API surface.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyIterator, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, op_ok, scene};

// ============================================================================
// Physics: add (rigid body / soft body / cloth / collision / fluid)
// ============================================================================

// The original Python `_h_phys_add` also accepted a nested `params` dict that
// got merged over the flat fields before use (`merged.update(extra)`). That
// key is not part of the schema `mcp_catalog.rs` advertises and has no test
// coverage; it is not ported here rather than adding an undocumented,
// unverified parameter surface. Flagged for the operator.

/// One entry of `phys_add`'s batch `physics` list. Not covered by the parity
/// test (only top-level tool schemas are pinned there), so this carries
/// `fluid_type` too even though the original catalog's nested item schema
/// omitted it — both modes call the same `phys_add_one`, so there is no
/// reason batch mode should be less capable than single mode.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysAddEntry {
    /// Name of an existing object in bpy.data.objects to attach physics to.
    pub name: String,
    /// Physics system to attach: RIGID_BODY, SOFT_BODY, CLOTH, COLLISION or
    /// FLUID. Case-insensitive; anything else is an error for this entry.
    #[serde(rename = "type")]
    pub r#type: String,
    /// RIGID_BODY only. ACTIVE (simulated, falls) or PASSIVE (static collider).
    /// Default ACTIVE.
    pub body_type: Option<String>,
    /// Mass in kilograms. RIGID_BODY and SOFT_BODY default 1.0, CLOTH default
    /// 0.3. Ignored for COLLISION and FLUID.
    pub mass: Option<f64>,
    /// Surface friction, 0..1 (0 = frictionless). Default 0.5. RIGID_BODY and
    /// SOFT_BODY only.
    pub friction: Option<f64>,
    /// Restitution (bounce), 0..1 (0 = no bounce). Default 0.0. RIGID_BODY only.
    pub bounciness: Option<f64>,
    /// RIGID_BODY collision shape. Blender 5.2 accepts BOX, SPHERE, CAPSULE,
    /// CYLINDER, CONE, CONVEX_HULL, MESH, COMPOUND. Default CONVEX_HULL.
    pub shape: Option<String>,
    /// FLUID only. Role of this object in a Mantaflow setup: DOMAIN, FLOW,
    /// EFFECTOR or NONE. Default FLOW.
    pub fluid_type: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysAdd {
    /// Single mode: name of an existing object. Ignored when `physics` is given.
    pub name: Option<String>,
    /// Single mode: physics system to attach — RIGID_BODY, SOFT_BODY, CLOTH,
    /// COLLISION or FLUID. Case-insensitive; anything else is an error.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// RIGID_BODY only. ACTIVE (simulated, falls) or PASSIVE (static collider).
    /// Default ACTIVE.
    pub body_type: Option<String>,
    /// Mass in kilograms. RIGID_BODY and SOFT_BODY default 1.0, CLOTH default
    /// 0.3. Ignored for COLLISION and FLUID.
    pub mass: Option<f64>,
    /// Surface friction, 0..1 (0 = frictionless). Default 0.5. RIGID_BODY and
    /// SOFT_BODY only.
    pub friction: Option<f64>,
    /// Restitution (bounce), 0..1 (0 = no bounce). Default 0.0. RIGID_BODY only.
    pub bounciness: Option<f64>,
    /// RIGID_BODY collision shape. Blender 5.2 accepts BOX, SPHERE, CAPSULE,
    /// CYLINDER, CONE, CONVEX_HULL, MESH, COMPOUND. Default CONVEX_HULL.
    pub shape: Option<String>,
    /// FLUID only. Role of this object in a Mantaflow setup: DOMAIN, FLOW,
    /// EFFECTOR or NONE. Default FLOW.
    pub fluid_type: Option<String>,
    /// Batch mode: one entry per object, e.g.
    /// [{"name": "Cube", "type": "RIGID_BODY", "mass": 2.0}, ...]. When present,
    /// the flat single-mode fields above are ignored and per-object failures are
    /// returned as data in `results` instead of failing the whole call.
    pub physics: Option<Vec<PhysAddEntry>>,
}

#[allow(clippy::too_many_arguments)]
fn phys_add_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    name: &str,
    r#type: &str,
    body_type: Option<&str>,
    mass: Option<f64>,
    friction: Option<f64>,
    bounciness: Option<f64>,
    shape: Option<&str>,
    fluid_type: Option<&str>,
) -> PyResult<Value> {
    let d = data(bpy)?;
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Ok(json!({ "name": name, "error": "not found" }));
    }
    let phys_type = r#type.to_uppercase();
    bpy.getattr("context")?
        .getattr("view_layer")?
        .getattr("objects")?
        .setattr("active", &obj)?;

    let r = (|| -> PyResult<()> {
        match phys_type.as_str() {
            "RIGID_BODY" => {
                let result = bpy
                    .getattr("ops")?
                    .getattr("rigidbody")?
                    .call_method0("object_add")?;
                // A failure here lands in `r`, which the `match r` below already
                // turns into this item's `{"error": ...}` — the same per-item
                // batch handling every other branch's `?` already relies on.
                op_ok(&result, &format!("add rigid body to {name}"))?;
                let rb = obj.getattr("rigid_body")?;
                rb.setattr("type", body_type.unwrap_or("ACTIVE"))?;
                rb.setattr("mass", mass.unwrap_or(1.0))?;
                rb.setattr("friction", friction.unwrap_or(0.5))?;
                rb.setattr("restitution", bounciness.unwrap_or(0.0))?;
                rb.setattr("collision_shape", shape.unwrap_or("CONVEX_HULL"))?;
            }
            "SOFT_BODY" => {
                let kw = PyDict::new(py);
                kw.set_item("name", "Softbody")?;
                kw.set_item("type", "SOFT_BODY")?;
                obj.getattr("modifiers")?
                    .call_method("new", (), Some(&kw))?;
                let sb = obj
                    .getattr("modifiers")?
                    .call_method1("get", ("Softbody",))?
                    .getattr("settings")?;
                sb.setattr("mass", mass.unwrap_or(1.0))?;
                sb.setattr("friction", friction.unwrap_or(0.5))?;
            }
            "CLOTH" => {
                let kw = PyDict::new(py);
                kw.set_item("name", "Cloth")?;
                kw.set_item("type", "CLOTH")?;
                obj.getattr("modifiers")?
                    .call_method("new", (), Some(&kw))?;
                let cloth = obj
                    .getattr("modifiers")?
                    .call_method1("get", ("Cloth",))?
                    .getattr("settings")?;
                cloth.setattr("mass", mass.unwrap_or(0.3))?;
            }
            "COLLISION" => {
                let kw = PyDict::new(py);
                kw.set_item("name", "Collision")?;
                kw.set_item("type", "COLLISION")?;
                obj.getattr("modifiers")?
                    .call_method("new", (), Some(&kw))?;
            }
            "FLUID" => {
                let kw = PyDict::new(py);
                kw.set_item("name", "Fluid")?;
                kw.set_item("type", "FLUID")?;
                obj.getattr("modifiers")?
                    .call_method("new", (), Some(&kw))?;
                obj.getattr("modifiers")?
                    .call_method1("get", ("Fluid",))?
                    .setattr("fluid_type", fluid_type.unwrap_or("FLOW"))?;
            }
            // The Python original had no `else`: an unrecognized type silently
            // added nothing and still reported success, so the model believed it
            // had set up physics that do not exist. Say so instead.
            other => {
                return Err(PyValueError::new_err(format!(
                    "unknown physics type: {other} \
                     (expected RIGID_BODY, SOFT_BODY, CLOTH, COLLISION or FLUID)"
                )));
            }
        }
        Ok(())
    })();

    match r {
        Ok(()) => Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "physics_type": phys_type,
        })),
        Err(e) => Ok(json!({ "name": name, "error": format!("{e}") })),
    }
}

/// Promote a batch-item-shaped `{"error": ...}` result into a hard `Err`, for use at
/// the TOP LEVEL of a single-target call. `phys_add_one`/`phys_remove_one` always
/// return `Ok` with either a success object or an `{"error": ...}` object, because
/// that is what batch mode needs to push into `results`. But when only one target
/// was ever named, that same value becomes the whole call's result, and per the MCP
/// error contract a call that could not do what it was asked must return `Err`.
///
/// "not found" is bad input (`PyValueError`); anything else is an operation that
/// failed inside Blender (`PyRuntimeError`).
fn require_ok(name: &str, result: Value) -> PyResult<Value> {
    match result.get("error").and_then(Value::as_str) {
        Some("not found") => Err(PyValueError::new_err(format!("object not found: {name}"))),
        Some(msg) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "{name}: {msg}"
        ))),
        None => Ok(result),
    }
}

crate::tool!(
    "phys_add",
    "Attach a physics system to existing objects: RIGID_BODY, SOFT_BODY, CLOTH, \
     COLLISION or FLUID (Mantaflow). Use `name`+`type` for one object or `physics` \
     for a batch. Simulate the result with phys_bake (point caches); undo it with \
     phys_remove. Unrelated to geometry-nodes simulation zones (sim_create_zone, \
     sim_bake).",
    PhysAdd,
    |py: Python<'_>, p: PhysAdd| {
        let bpy = py.import("bpy")?;

        if let Some(entries) = p.physics {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for e in entries {
                let r = phys_add_one(
                    py,
                    &bpy,
                    e.name.as_str(),
                    e.r#type.as_str(),
                    e.body_type.as_deref(),
                    e.mass,
                    e.friction,
                    e.bounciness,
                    e.shape.as_deref(),
                    e.fluid_type.as_deref(),
                )?;
                if r.get("error").is_none() {
                    ok += 1;
                }
                results.push(r);
            }
            return Ok(json!({ "added": ok, "results": results }));
        }

        let name = p
            .name
            .ok_or_else(|| PyValueError::new_err("missing name"))?;
        let ty = p
            .r#type
            .ok_or_else(|| PyValueError::new_err("missing type"))?;
        let result = phys_add_one(
            py,
            &bpy,
            &name,
            &ty,
            p.body_type.as_deref(),
            p.mass,
            p.friction,
            p.bounciness,
            p.shape.as_deref(),
            p.fluid_type.as_deref(),
        )?;
        // Single-target mode: `phys_add_one`'s `{"error": ...}` shape is only valid
        // as a batch-item's DATA (see the `physics` branch above). At the top level
        // of the call it must become a hard `Err`, or the model reads past it.
        require_ok(&name, result)
    }
);

// ============================================================================
// Physics: remove
// ============================================================================

fn phys_remove_one(bpy: &Bound<'_, PyAny>, name: &str) -> PyResult<Value> {
    let d = data(bpy)?;
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Ok(json!({ "name": name, "error": "not found" }));
    }
    bpy.getattr("context")?
        .getattr("view_layer")?
        .getattr("objects")?
        .setattr("active", &obj)?;

    let rb = obj.getattr("rigid_body")?;
    if !rb.is_none() {
        let result = bpy
            .getattr("ops")?
            .getattr("rigidbody")?
            .call_method0("object_remove")?;
        // This function feeds both single-target (`require_ok` promotes an
        // `{"error": ...}` to a hard `Err`) and batch mode (`.collect::<PyResult<_>>()`
        // aborts the whole batch on any `Err`), so a decline is folded into the
        // same soft-error shape `obj.is_none()` above already uses, not a raw `?`.
        if let Err(e) = op_ok(&result, &format!("remove rigid body from {name}")) {
            return Ok(json!({ "name": name, "error": e.to_string() }));
        }
    }

    let mods = obj.getattr("modifiers")?;
    let mut doomed = Vec::new();
    for m in PyIterator::from_object(&mods)? {
        let m = m?;
        let ty: String = m.getattr("type")?.extract()?;
        if matches!(ty.as_str(), "SOFT_BODY" | "CLOTH" | "FLUID" | "COLLISION") {
            doomed.push(m);
        }
    }
    for m in doomed {
        mods.call_method1("remove", (&m,))?;
    }

    Ok(json!({
        "name": obj.getattr("name")?.extract::<String>()?,
        "physics_removed": true,
    }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysRemove {
    /// Single mode: name of one object to strip physics from. Ignored when
    /// `names` is given.
    pub name: Option<String>,
    /// Batch mode: list of object names. Per-object failures come back as data
    /// in `results` instead of failing the whole call.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "phys_remove",
    "Strip every physics system this registry can add off an object: rigid body \
     plus the SOFT_BODY, CLOTH, FLUID and COLLISION modifiers. The inverse of \
     phys_add; does not touch particle systems (phys_remove_particles) or \
     geometry-nodes simulation caches (sim_clear_cache).",
    PhysRemove,
    |py: Python<'_>, p: PhysRemove| {
        let bpy = py.import("bpy")?;
        let names: Vec<String> = p
            .names
            .or_else(|| p.name.map(|n| vec![n]))
            .unwrap_or_default();

        let mut results: Vec<Value> = names
            .iter()
            .map(|n| phys_remove_one(&bpy, n))
            .collect::<PyResult<_>>()?;

        if results.len() == 1 {
            // Single-target mode: same reasoning as `phys_add`'s `require_ok` call —
            // this one result IS the whole call's result, so a `{"error": ...}"`
            // shape must become a hard `Err` here instead of silently returning it.
            return require_ok(&names[0], results.remove(0));
        }
        let removed = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "removed": removed, "results": results }))
    }
);

// ============================================================================
// Physics: point-cache bake
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysBake {
    /// First frame to bake. Integer; default: the scene's current frame_start.
    /// Applied to the scene for the duration of the bake and restored after.
    pub frame_start: Option<i32>,
    /// Last frame to bake. Integer; default: the scene's current frame_end.
    /// Applied to the scene for the duration of the bake and restored after.
    pub frame_end: Option<i32>,
}

crate::tool!(
    "phys_bake",
    "Bake every classic point cache in the scene at once \
     (bpy.ops.ptcache.bake_all): rigid body, cloth, soft body, particles and \
     Mantaflow fluid — the systems phys_add creates. Scene-wide, no object \
     argument. For a geometry-nodes simulation zone use sim_bake instead, which \
     bakes one object's zone cache; sim_clear_cache deletes that zone cache.",
    PhysBake,
    |py: Python<'_>, p: PhysBake| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;

        let old_start: i32 = s.getattr("frame_start")?.extract()?;
        let old_end: i32 = s.getattr("frame_end")?.extract()?;
        let frame_start = p.frame_start.unwrap_or(old_start);
        let frame_end = p.frame_end.unwrap_or(old_end);

        // `ptcache.bake_all` bakes each point cache over the SCENE's frame range,
        // so the requested range has to be driven through the scene. The Python
        // original echoed frame_start/frame_end back in the result and never applied
        // them — the caller was told a range that had not been baked.
        s.setattr("frame_start", frame_start)?;
        s.setattr("frame_end", frame_end)?;

        let baked = (|| -> PyResult<()> {
            let kw = PyDict::new(py);
            kw.set_item("bake", true)?;
            let result =
                bpy.getattr("ops")?
                    .getattr("ptcache")?
                    .call_method("bake_all", (), Some(&kw))?;
            op_ok(&result, "bake simulation")
        })();

        // Restore the scene whatever happened — a bake must not leave the frame
        // range mutated behind it.
        s.setattr("frame_start", old_start)?;
        s.setattr("frame_end", old_end)?;
        baked?;

        Ok(json!({ "frame_start": frame_start, "frame_end": frame_end, "baked": true }))
    }
);

// ============================================================================
// Particles
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysAddParticles {
    /// Name of the existing mesh object to emit particles from.
    pub name: String,
    /// Name to give the new particle system slot. Default "ParticleSystem".
    pub ps_name: Option<String>,
    /// Number of particles emitted over the emission range. Integer, default
    /// 1000.
    pub count: Option<i32>,
    /// Particle lifetime in frames. Default 50.0.
    pub lifetime: Option<f64>,
    /// Where particles are born on the emitter. Blender 5.2: VERT, FACE,
    /// VOLUME. Default FACE.
    pub emit_from: Option<String>,
    /// Particle solver. Blender 5.2: NO (none), NEWTON, KEYED, BOIDS, FLUID.
    /// Default NEWTON.
    pub physics_type: Option<String>,
    /// How particles are drawn/rendered. Blender 5.2: NONE, HALO, LINE, PATH,
    /// OBJECT, COLLECTION. Default: Blender's own (HALO); left untouched when
    /// omitted.
    pub render_type: Option<String>,
    /// Name of an object to instance on every particle. Setting it forces
    /// render_type to OBJECT, overriding `render_type` above.
    pub instance_object: Option<String>,
}

crate::tool!(
    "phys_add_particles",
    "Add a particle system to a mesh object and set its core emission settings \
     (count, lifetime, emit_from, solver, render type). Tune an existing system \
     with phys_update_particles; delete one with phys_remove_particles. Particle \
     caches are baked by phys_bake.",
    PhysAddParticles,
    |py: Python<'_>, p: PhysAddParticles| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        let result = bpy
            .getattr("ops")?
            .getattr("object")?
            .call_method0("particle_system_add")?;
        op_ok(&result, &format!("add particle system to {}", p.name))?;

        let systems = obj.getattr("particle_systems")?;
        let ps = systems.get_item(-1)?;
        let ps_name = p.ps_name.unwrap_or_else(|| "ParticleSystem".to_string());
        ps.setattr("name", ps_name.as_str())?;

        let settings = ps.getattr("settings")?;
        settings.setattr("count", p.count.unwrap_or(1000))?;
        settings.setattr("lifetime", p.lifetime.unwrap_or(50.0))?;
        settings.setattr("emit_from", p.emit_from.as_deref().unwrap_or("FACE"))?;
        settings.setattr(
            "physics_type",
            p.physics_type.as_deref().unwrap_or("NEWTON"),
        )?;

        if let Some(rt) = p.render_type.as_deref() {
            settings.setattr("render_type", rt)?;
        }
        if let Some(io) = p.instance_object.as_deref() {
            settings.setattr("render_type", "OBJECT")?;
            let iobj = d.getattr("objects")?.call_method1("get", (io,))?;
            settings.setattr("instance_object", &iobj)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "particle_system": ps.getattr("name")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysRemoveParticles {
    /// Name of the object carrying the particle system.
    pub name: String,
    /// Particle system to remove, by name. Default: the object's currently
    /// active particle system.
    pub ps_name: Option<String>,
}

crate::tool!(
    "phys_remove_particles",
    "Remove one particle system from an object — by name, or the active one when \
     no name is given. The inverse of phys_add_particles; phys_remove does not \
     touch particle systems.",
    PhysRemoveParticles,
    |py: Python<'_>, p: PhysRemoveParticles| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }

        if let Some(ps_name) = p.ps_name.as_deref() {
            let idx: i32 = obj
                .getattr("particle_systems")?
                .call_method1("find", (ps_name,))?
                .extract()?;
            if idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Particle system not found: {ps_name}"
                )));
            }
            obj.getattr("particle_systems")?
                .setattr("active_index", idx)?;
        }

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        let result = bpy
            .getattr("ops")?
            .getattr("object")?
            .call_method0("particle_system_remove")?;
        op_ok(&result, &format!("remove particle system from {}", p.name))?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "removed": p.ps_name.unwrap_or_else(|| "active".to_string()),
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysUpdateParticles {
    /// Name of the object carrying the particle system.
    pub name: String,
    /// Particle system to modify, by name. Default: the object's first particle
    /// system (index 0).
    pub ps_name: Option<String>,
    /// New particle count. Integer. Omit to leave unchanged.
    pub count: Option<i32>,
    /// New particle lifetime, in frames. Omit to leave unchanged.
    pub lifetime: Option<f64>,
    /// New emission source. Blender 5.2: VERT, FACE, VOLUME. Omit to leave
    /// unchanged.
    pub emit_from: Option<String>,
    /// New particle solver. Blender 5.2: NO, NEWTON, KEYED, BOIDS, FLUID. Omit
    /// to leave unchanged.
    pub physics_type: Option<String>,
    /// New render type. Blender 5.2: NONE, HALO, LINE, PATH, OBJECT,
    /// COLLECTION. Omit to leave unchanged.
    pub render_type: Option<String>,
}

crate::tool!(
    "phys_update_particles",
    "Change settings on an existing particle system in place — only the fields \
     you pass are written, the rest keep their current values. Create the system \
     first with phys_add_particles.",
    PhysUpdateParticles,
    |py: Python<'_>, p: PhysUpdateParticles| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }

        let systems = obj.getattr("particle_systems")?;
        // The original Python resolves the fallback via
        // `p.get("ps_name", obj.particle_systems[0].name)` — Python evaluates a
        // dict.get default *eagerly*, so `particle_systems[0]` is indexed even
        // when `ps_name` *was* supplied, raising IndexError on any object with
        // zero particle systems regardless of which branch was actually wanted.
        // Index 0 is only touched here when it is actually needed.
        let ps = match p.ps_name.as_deref() {
            Some(ps_name) => systems.call_method1("get", (ps_name,))?,
            None => {
                let first_name: String = systems.get_item(0)?.getattr("name")?.extract()?;
                systems.call_method1("get", (first_name.as_str(),))?
            }
        };
        if ps.is_none() {
            return Err(PyValueError::new_err("No particle system found"));
        }

        let settings = ps.getattr("settings")?;
        if let Some(v) = p.count {
            settings.setattr("count", v)?;
        }
        if let Some(v) = p.lifetime {
            settings.setattr("lifetime", v)?;
        }
        if let Some(v) = p.emit_from.as_deref() {
            settings.setattr("emit_from", v)?;
        }
        if let Some(v) = p.physics_type.as_deref() {
            settings.setattr("physics_type", v)?;
        }
        if let Some(v) = p.render_type.as_deref() {
            settings.setattr("render_type", v)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "particle_system": ps.getattr("name")?.extract::<String>()?,
        }))
    }
);

// ============================================================================
// Force fields
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PhysAddForceField {
    /// Force field type. Verified live on Blender 5.2 against
    /// bpy.ops.object.effector_add: FORCE, WIND, VORTEX, MAGNET, HARMONIC,
    /// CHARGE, LENNARDJ, TEXTURE, GUIDE, BOID, TURBULENCE, DRAG, FLUID.
    /// Case-insensitive. Legacy aliases are accepted and translated: MAGNETIC
    /// -> MAGNET, SMOKE_FLOW / FLUID_FLOW -> FLUID (the created object reports
    /// field.type FLUID_FLOW). There is no MAGNETIC or SMOKE_FLOW field in 5.2.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name for the new empty object carrying the field. Default: Blender's own
    /// ("Field").
    pub name: Option<String>,
    /// World-space location as [x, y, z] in metres. Shorter arrays are
    /// zero-padded, longer ones truncated. Default [0, 0, 0].
    pub location: Option<Vec<f64>>,
    /// Field strength. Sign flips the direction (negative = attract for FORCE).
    /// Default 1.0.
    pub strength: Option<f64>,
    /// Convert effector force into an air-flow/drag effect, 0..n. Default 0.0
    /// (pure force).
    pub flow: Option<f64>,
    /// WIND only: noise amount added to the wind direction. Default 0.0.
    /// Ignored for other types.
    pub noise: Option<f64>,
    /// VORTEX only: inward pull toward the vortex axis, -1..1. Default 0.0.
    /// Ignored for other types.
    pub inflow: Option<f64>,
    /// TURBULENCE only: size of the noise cells. Default 1.0. Ignored for other
    /// types.
    pub size: Option<f64>,
}

/// Fills `[x, y, z]` from a possibly-short/long slice, zero-padding or
/// truncating. Mirrors the tolerant handling `obj_create_primitive` already
/// uses for `location` elsewhere in the registry, instead of hard-erroring on
/// an odd-length array the way the unwrapped Python `effector_add(location=…)`
/// call would.
fn loc3(v: &[f64]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for (i, x) in v.iter().take(3).enumerate() {
        out[i] = *x;
    }
    out
}

/// KNOWN ISSUE fixed: the schema used to advertise `MAGNETIC` and `SMOKE_FLOW`.
/// Neither exists in Blender 5.2 — verified live, `bpy.ops.object.effector_add`
/// rejects both with `enum "MAGNETIC" not found in ('FORCE', 'WIND', 'VORTEX',
/// 'MAGNET', ...)`. The operator's live enum is FORCE, WIND, VORTEX, MAGNET,
/// HARMONIC, CHARGE, LENNARDJ, TEXTURE, GUIDE, BOID, TURBULENCE, DRAG, FLUID.
/// (`FieldSettings.type`, the *resulting* property, additionally carries NONE and
/// spells the fluid one `FLUID_FLOW` — the operator does not accept that
/// spelling.) The two dead names are translated rather than rejected, so a model
/// working from the old description still lands on a real field.
fn effector_type_alias(t: &str) -> &str {
    match t {
        "MAGNETIC" => "MAGNET",
        "SMOKE_FLOW" | "FLUID_FLOW" => "FLUID",
        other => other,
    }
}

crate::tool!(
    "phys_add_force_field",
    "Create a force-field effector object that pushes particles, cloth, soft \
     bodies and fluids around. Blender 5.2 types: FORCE, WIND, VORTEX, MAGNET, \
     HARMONIC, CHARGE, LENNARDJ, TEXTURE, GUIDE, BOID, TURBULENCE, DRAG, FLUID \
     (there is no MAGNETIC or SMOKE_FLOW — those are accepted as aliases for \
     MAGNET and FLUID). The field affects simulations added with phys_add and \
     phys_add_particles; bake them with phys_bake.",
    PhysAddForceField,
    |py: Python<'_>, p: PhysAddForceField| {
        let bpy = py.import("bpy")?;
        let field_type = effector_type_alias(&p.r#type.to_uppercase()).to_string();
        let loc = loc3(&p.location.unwrap_or_else(|| vec![0.0, 0.0, 0.0]));

        let kw = PyDict::new(py);
        kw.set_item("type", field_type.as_str())?;
        kw.set_item("location", PyTuple::new(py, loc)?)?;
        let result =
            bpy.getattr("ops")?
                .getattr("object")?
                .call_method("effector_add", (), Some(&kw))?;
        op_ok(&result, &format!("add {field_type} force field"))?;

        let obj = bpy.getattr("context")?.getattr("active_object")?;
        if let Some(name) = p.name.as_deref() {
            obj.setattr("name", name)?;
        }

        let field = obj.getattr("field")?;
        field.setattr("strength", p.strength.unwrap_or(1.0))?;
        field.setattr("flow", p.flow.unwrap_or(0.0))?;
        match field_type.as_str() {
            "WIND" => field.setattr("noise", p.noise.unwrap_or(0.0))?,
            "VORTEX" => field.setattr("inflow", p.inflow.unwrap_or(0.0))?,
            "TURBULENCE" => field.setattr("size", p.size.unwrap_or(1.0))?,
            _ => {}
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "type": field_type,
            "strength": field.getattr("strength")?.extract::<f64>()?,
        }))
    }
);

// ============================================================================
// Simulation zones (geometry nodes, Blender 4.0+)
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SimCreateZone {
    /// Name of an object whose first NODES (geometry nodes) modifier supplies
    /// the tree to edit. Used only when `node_group` is omitted.
    pub object: Option<String>,
    /// Name of a node group in bpy.data.node_groups to edit directly. Takes
    /// precedence over `object`. Give one of the two.
    pub node_group: Option<String>,
}

crate::tool!(
    "sim_create_zone",
    "Add a paired Simulation Input/Output zone to a geometry node tree and wire \
     its Geometry sockets (Blender 4.0+). This is the geometry-nodes simulation \
     system, not the classic physics of phys_add: bake it with sim_bake, point \
     its cache directory with sim_set_params, drop the cache with sim_clear_cache.",
    SimCreateZone,
    |py: Python<'_>, p: SimCreateZone| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let mut tree: Option<Bound<'_, PyAny>> = None;
        if let Some(ng) = p.node_group.as_deref() {
            let t = d.getattr("node_groups")?.call_method1("get", (ng,))?;
            if !t.is_none() {
                tree = Some(t);
            }
        } else if let Some(obj_name) = p.object.as_deref() {
            let obj = d.getattr("objects")?.call_method1("get", (obj_name,))?;
            if !obj.is_none() {
                for m in PyIterator::from_object(&obj.getattr("modifiers")?)? {
                    let m = m?;
                    let ty: String = m.getattr("type")?.extract()?;
                    if ty != "NODES" {
                        continue;
                    }
                    let ng = m.getattr("node_group")?;
                    if !ng.is_none() {
                        tree = Some(ng);
                        break;
                    }
                }
            }
        }

        let Some(tree) = tree else {
            return Err(PyValueError::new_err("no geometry node tree found"));
        };

        let version: (i64, i64, i64) = bpy.getattr("app")?.getattr("version")?.extract()?;
        if version < (4, 0, 0) {
            return Err(PyValueError::new_err(
                "simulation nodes require Blender 4.0+",
            ));
        }

        let r = (|| -> PyResult<Value> {
            let nodes = tree.getattr("nodes")?;
            let sim_in = nodes.call_method1("new", ("GeometryNodeSimulationInput",))?;
            let sim_out = nodes.call_method1("new", ("GeometryNodeSimulationOutput",))?;
            sim_in.setattr("location", PyTuple::new(py, [-200.0, 0.0])?)?;
            sim_out.setattr("location", PyTuple::new(py, [200.0, 0.0])?)?;

            // KNOWN ISSUE fixed: a freshly-created `GeometryNodeSimulationInput` carries
            // NO Geometry socket at all until it is paired with its Output — verified
            // live: before `pair_with_output`, `sim_in.inputs` is empty and
            // `sim_in.outputs` is just `Delta Time`. Skipping this call (as the code
            // previously did) left `sim_in.paired_output` at `None`: the two nodes were
            // never actually a zone, just two nodes that happened to sit in the same
            // tree. It has to run before the Geometry sockets linked below exist.
            sim_in.call_method1("pair_with_output", (&sim_out,))?;

            // Wire the zone's own Geometry pass-through, by NAME. `outputs[0]` is
            // `Delta Time`, not `Geometry` — index-based linking (the previous code)
            // wired Delta Time -> Skip, a float into a boolean socket. `tree.links.new`
            // did not reject it, so the bug produced a tree that looked wired and was
            // not a working simulation zone.
            tree.getattr("links")?.call_method1(
                "new",
                (
                    sim_in.getattr("outputs")?.get_item("Geometry")?,
                    sim_out.getattr("inputs")?.get_item("Geometry")?,
                ),
            )?;

            Ok(json!({
                "node_group": tree.getattr("name")?.extract::<String>()?,
                "sim_input": sim_in.getattr("name")?.extract::<String>()?,
                "sim_output": sim_out.getattr("name")?.extract::<String>()?,
            }))
        })();

        match r {
            Ok(v) => Ok(v),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "failed to create simulation zone: {e}"
            ))),
        }
    }
);

/// The Python original also advertised `node_group` and `use_cache`. It read them
/// into locals and never used either one — the only thing this tool actually writes
/// is the modifier's `bake_directory`. They are not advertised here: a parameter the
/// model can set and we then ignore is a lie the schema tells on our behalf.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct SimSetParams {
    /// Name of the object carrying the geometry-nodes modifier. Required in
    /// practice: the call errors without it.
    pub object: Option<String>,
    /// Directory the modifier bakes its simulation cache into (the modifier's
    /// `bake_directory`). Blender's `//` prefix means relative to the saved
    /// .blend. Omit to leave the current directory unchanged.
    pub cache_path: Option<String>,
}

crate::tool!(
    "sim_set_params",
    "Point a geometry-nodes modifier's simulation bake cache at a directory. \
     Operates on the object's first NODES modifier. Affects what sim_bake writes \
     and sim_clear_cache deletes; unrelated to classic point caches (phys_bake).",
    SimSetParams,
    |py: Python<'_>, p: SimSetParams| {
        let bpy = py.import("bpy")?;
        let Some(obj_name) = p.object else {
            return Err(PyValueError::new_err("object required"));
        };
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (obj_name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {obj_name}"
            )));
        }

        let mut modifier: Option<Bound<'_, PyAny>> = None;
        for m in PyIterator::from_object(&obj.getattr("modifiers")?)? {
            let m = m?;
            let ty: String = m.getattr("type")?.extract()?;
            if ty == "NODES" {
                modifier = Some(m);
                break;
            }
        }
        let Some(modifier) = modifier else {
            return Err(PyValueError::new_err(format!(
                "no geometry nodes modifier found on {obj_name}"
            )));
        };

        let mut result = json!({
            "object": obj_name,
            "modifier": modifier.getattr("name")?.extract::<String>()?,
        });
        if let Some(cache_path) = p.cache_path.as_deref()
            && modifier.hasattr("bake_directory")?
        {
            modifier.setattr("bake_directory", cache_path)?;
            result["cache_path"] = json!(cache_path);
        }
        Ok(result)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SimBake {
    /// Name of the object whose geometry-nodes simulation zone is baked. Must
    /// carry a NODES modifier.
    pub object: String,
    /// First frame to bake. Integer; default: the scene's current frame_start.
    /// Driven through the scene for the duration of the bake, then restored.
    pub start_frame: Option<i32>,
    /// Last frame to bake. Integer; default: the scene's current frame_end.
    /// Driven through the scene for the duration of the bake, then restored.
    pub end_frame: Option<i32>,
}

crate::tool!(
    "sim_bake",
    "Bake one object's geometry-nodes simulation zone to its modifier cache \
     (bpy.ops.object.simulation_nodes_cache_bake). Per-object and \
     geometry-nodes-only — the scene-wide classic point caches (rigid body, \
     cloth, particles, fluid) are baked by phys_bake instead. Delete what this \
     writes with sim_clear_cache.",
    SimBake,
    |py: Python<'_>, p: SimBake| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let old_start: i32 = s.getattr("frame_start")?.extract()?;
        let old_end: i32 = s.getattr("frame_end")?.extract()?;
        let start = p.start_frame.unwrap_or(old_start);
        let end = p.end_frame.unwrap_or(old_end);

        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.object
            )));
        }

        let mut modifier: Option<Bound<'_, PyAny>> = None;
        for m in PyIterator::from_object(&obj.getattr("modifiers")?)? {
            let m = m?;
            let ty: String = m.getattr("type")?.extract()?;
            if ty == "NODES" {
                modifier = Some(m);
                break;
            }
        }
        if modifier.is_none() {
            return Err(PyValueError::new_err(format!(
                "no geometry nodes modifier found on {}",
                p.object
            )));
        }

        // KNOWN ISSUE fixed: the original Python reported `"frames":
        // "{start}-{end}"` without ever applying either value anywhere —
        // `simulation_nodes_cache_bake` takes no frame-range argument and
        // always bakes whatever the scene's current frame range is. Here the
        // scene range is genuinely driven by start_frame/end_frame for the
        // duration of the bake, then restored, so the reported range is now
        // true instead of a label pasted over the scene's actual range.
        let obj_name = p.object.clone();
        let r = (|| -> PyResult<Value> {
            s.setattr("frame_start", start)?;
            s.setattr("frame_end", end)?;

            let object_ops = bpy.getattr("ops")?.getattr("object")?;
            // Not checked: this is a reset step, and `select_all(DESELECT)`
            // legitimately returns {'CANCELLED'} when nothing was selected to
            // begin with (verified live) — the real action below is what matters.
            let deselect = PyDict::new(py);
            deselect.set_item("action", "DESELECT")?;
            object_ops.call_method("select_all", (), Some(&deselect))?;
            obj.call_method1("select_set", (true,))?;
            bpy.getattr("context")?
                .getattr("view_layer")?
                .getattr("objects")?
                .setattr("active", &obj)?;

            if object_ops.hasattr("simulation_nodes_cache_bake")? {
                let kw = PyDict::new(py);
                kw.set_item("selected", true)?;
                let result =
                    object_ops.call_method("simulation_nodes_cache_bake", (), Some(&kw))?;
                op_ok(&result, &format!("bake simulation cache for {obj_name}"))?;
                Ok(json!({
                    "object": obj_name,
                    "baked": true,
                    "frames": format!("{start}-{end}"),
                }))
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err(
                    "simulation baking is not available in this Blender version",
                ))
            }
        })();

        // Restore the scene's frame range whatever happened, then let a failure
        // propagate as a failure.
        s.setattr("frame_start", old_start)?;
        s.setattr("frame_end", old_end)?;
        r
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SimClearCache {
    /// Name of the object whose geometry-nodes simulation cache is deleted.
    pub object: String,
}

crate::tool!(
    "sim_clear_cache",
    "Delete one object's baked geometry-nodes simulation cache \
     (bpy.ops.object.simulation_nodes_cache_delete) — the exact thing sim_bake \
     wrote. Does not touch classic point caches; re-bake those with phys_bake.",
    SimClearCache,
    |py: Python<'_>, p: SimClearCache| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.object
            )));
        }

        let obj_name = p.object.clone();

        (|| -> PyResult<Value> {
            let object_ops = bpy.getattr("ops")?.getattr("object")?;
            // Not checked: reset step, see the identical comment in sim_bake.
            let deselect = PyDict::new(py);
            deselect.set_item("action", "DESELECT")?;
            object_ops.call_method("select_all", (), Some(&deselect))?;
            obj.call_method1("select_set", (true,))?;
            bpy.getattr("context")?
                .getattr("view_layer")?
                .getattr("objects")?
                .setattr("active", &obj)?;

            if object_ops.hasattr("simulation_nodes_cache_delete")? {
                let kw = PyDict::new(py);
                kw.set_item("selected", true)?;
                let result =
                    object_ops.call_method("simulation_nodes_cache_delete", (), Some(&kw))?;
                op_ok(&result, &format!("clear simulation cache for {obj_name}"))?;
                Ok(json!({ "object": obj_name, "cache_cleared": true }))
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err(
                    "cache clearing is not available in this Blender version",
                ))
            }
        })()
    }
);

// ============================================================================
// Render queue
// ============================================================================
//
// Jobs persist as a JSON array in the scene custom property
// `mcp_render_queue`, and job ids come from a separate monotonic counter in
// `mcp_render_queue_next_id`. The original Python stored jobs as plain nested
// dicts directly in the same scene custom property (Blender turns that into
// nested `IDPropertyGroup`/`IDPropertyArray` objects). Round-tripping that
// through PyO3 would mean a fragile recursive walk to convert
// `IDPropertyGroup` <-> `serde_json::Value` on every read and write; storing
// one JSON string instead makes the (de)serialization exactly
// `serde_json::to_string`/`from_str`, with Blender only ever seeing an opaque
// string custom property.

fn render_queue_get(scene: &Bound<'_, PyAny>) -> PyResult<Vec<Value>> {
    let raw: Option<String> = scene
        .get_item("mcp_render_queue")
        .ok()
        .and_then(|v| v.extract().ok());
    Ok(raw
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default())
}

fn render_queue_set(scene: &Bound<'_, PyAny>, queue: &[Value]) -> PyResult<()> {
    let s = serde_json::to_string(queue)
        .map_err(|e| PyValueError::new_err(format!("serialize render queue: {e}")))?;
    scene.set_item("mcp_render_queue", s.as_str())?;
    Ok(())
}

/// KNOWN ISSUE fixed: the original Python assigned each new job
/// `id = len(queue) + 1`, which reissues a deleted job's id the moment the
/// queue shrinks — e.g. add jobs 1..3, clear, add again: the new job is also
/// `1`, colliding with a ticket a caller may still be holding. A separate,
/// never-reused, scene-persisted counter fixes this without changing the
/// advertised schema.
fn next_render_job_id(scene: &Bound<'_, PyAny>) -> PyResult<i64> {
    let next: i64 = scene
        .get_item("mcp_render_queue_next_id")
        .ok()
        .and_then(|v| v.extract().ok())
        .unwrap_or(1);
    scene.set_item("mcp_render_queue_next_id", next + 1)?;
    Ok(next)
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RenderQueueAdd {
    /// Output path written to render.filepath when the job runs. Frames are
    /// numbered onto it; Blender's `//` prefix means relative to the saved
    /// .blend.
    pub output_path: String,
    /// First frame of the job. Integer; default: the scene's frame_start at the
    /// time the job is queued.
    pub start_frame: Option<i32>,
    /// Last frame of the job. Integer; default: the scene's frame_end at the
    /// time the job is queued.
    pub end_frame: Option<i32>,
    /// Name of a camera object to render through. Omit to keep the scene's
    /// active camera. A name that does not resolve is ignored.
    pub camera: Option<String>,
    /// Output width in pixels. Omit to keep the scene's current value.
    pub resolution_x: Option<i32>,
    /// Output height in pixels. Omit to keep the scene's current value.
    pub resolution_y: Option<i32>,
    /// Cycles sample count (scene.cycles.samples). Integer. Omit to keep the
    /// scene's current value.
    pub samples: Option<i32>,
    /// Render engine for this job, e.g. EEVEE, CYCLES, WORKBENCH. The friendly
    /// name is resolved against this Blender's real engine ids (5.2:
    /// BLENDER_EEVEE, BLENDER_WORKBENCH, CYCLES). Omit to keep the current
    /// engine.
    pub engine: Option<String>,
}

crate::tool!(
    "render_queue_add",
    "Queue a render job (output path, frame range, camera, resolution, samples, \
     engine) for later execution by render_queue_start. Queues only — nothing \
     renders here. For a single immediate render use scene_render; for settings \
     that persist on the scene use render_set.",
    RenderQueueAdd,
    |py: Python<'_>, p: RenderQueueAdd| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let start_frame = match p.start_frame {
            Some(v) => v,
            None => s.getattr("frame_start")?.extract()?,
        };
        let end_frame = match p.end_frame {
            Some(v) => v,
            None => s.getattr("frame_end")?.extract()?,
        };

        let id = next_render_job_id(&s)?;
        let job = json!({
            "id": id,
            "output_path": p.output_path,
            "start_frame": start_frame,
            "end_frame": end_frame,
            "camera": p.camera,
            "resolution_x": p.resolution_x,
            "resolution_y": p.resolution_y,
            "samples": p.samples,
            "engine": p.engine,
            "status": "pending",
        });

        let mut queue = render_queue_get(&s)?;
        queue.push(job.clone());
        render_queue_set(&s, &queue)?;

        Ok(json!({ "added": job }))
    }
);

/// `render_queue_list` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct RenderQueueList {}

crate::tool!(
    "render_queue_list",
    "List every job in the scene's render queue with its id, settings and status \
     (pending / completed / \"failed: ...\"). Takes no arguments. Jobs are added \
     by render_queue_add and run by render_queue_start.",
    RenderQueueList,
    |py: Python<'_>, _p: RenderQueueList| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let queue = render_queue_get(&s)?;
        let count = queue.len();
        Ok(json!({ "jobs": queue, "count": count }))
    }
);

/// The Python original advertised a `background` flag and never read it. Rendering
/// here is synchronous on Blender's main thread — real background rendering would
/// mean spawning a separate `blender --background` process. Rather than let the
/// model ask for something it silently will not get, the flag is not advertised, and
/// the description says plainly that the call blocks.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct RenderQueueStart {}

crate::tool!(
    "render_queue_start",
    "Render every pending job queued by render_queue_add, in order, applying each \
     job's settings to the scene as it goes. Takes no arguments and blocks Blender \
     until all jobs finish; one job's failure is recorded on that job and does not \
     stop the rest. Note that the applied settings are left on the scene \
     afterwards — unlike scene_render, which restores them.",
    RenderQueueStart,
    |py: Python<'_>, _p: RenderQueueStart| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let mut queue = render_queue_get(&s)?;

        let pending: Vec<usize> = queue
            .iter()
            .enumerate()
            .filter(|(_, j)| j.get("status").and_then(Value::as_str) == Some("pending"))
            .map(|(i, _)| i)
            .collect();
        let total = pending.len();

        // An empty queue is a truthful, well-formed empty result — not a failure —
        // and matches the success shape below (`{"completed": [...], "total": N}`).
        if pending.is_empty() {
            return Ok(json!({ "completed": [], "total": 0 }));
        }

        let render = s.getattr("render")?;
        let d = data(&bpy)?;
        let mut completed = Vec::new();

        for idx in pending {
            let job = queue[idx].clone();
            let output_path = job
                .get("output_path")
                .and_then(Value::as_str)
                .unwrap_or_default();
            render.setattr("filepath", output_path)?;
            if let Some(v) = job.get("start_frame").and_then(Value::as_i64) {
                s.setattr("frame_start", v as i32)?;
            }
            if let Some(v) = job.get("end_frame").and_then(Value::as_i64) {
                s.setattr("frame_end", v as i32)?;
            }
            if let Some(cam) = job.get("camera").and_then(Value::as_str) {
                let cam_obj = d.getattr("objects")?.call_method1("get", (cam,))?;
                if !cam_obj.is_none() {
                    s.setattr("camera", &cam_obj)?;
                }
            }
            if let Some(v) = job.get("resolution_x").and_then(Value::as_i64) {
                render.setattr("resolution_x", v as i32)?;
            }
            if let Some(v) = job.get("resolution_y").and_then(Value::as_i64) {
                render.setattr("resolution_y", v as i32)?;
            }
            if let Some(v) = job.get("samples").and_then(Value::as_i64)
                && let Ok(cycles) = s.getattr("cycles")
            {
                cycles.setattr("samples", v as i32)?;
            }
            if let Some(engine) = job.get("engine").and_then(Value::as_str) {
                render.setattr("engine", super::bpy::render_engine(&s, engine)?)?;
            }

            let kw = PyDict::new(py);
            kw.set_item("animation", true)?;
            let job_id = job.get("id").and_then(Value::as_i64).unwrap_or(-1);
            // Mirrors the original Python's per-job try/except: one job's
            // render failure does not abort the rest of the queue. A decline
            // (Ok(CANCELLED), never actually observed live for render.render but
            // checked for consistency) is folded into the same per-job "failed"
            // status as a raised error, not silently marked "completed".
            match bpy
                .getattr("ops")?
                .getattr("render")?
                .call_method("render", (), Some(&kw))
                .and_then(|r| op_ok(&r, "render"))
            {
                Ok(()) => {
                    queue[idx]["status"] = json!("completed");
                    completed.push(job_id);
                }
                Err(e) => {
                    queue[idx]["status"] = json!(format!("failed: {e}"));
                }
            }
        }

        render_queue_set(&s, &queue)?;
        Ok(json!({ "completed": completed, "total": total }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RenderQueueClear {
    /// true: drop only jobs whose status is "completed", keeping pending and
    /// failed ones. false (default): empty the queue entirely.
    pub completed_only: Option<bool>,
}

crate::tool!(
    "render_queue_clear",
    "Empty the render queue, or with completed_only=true drop just the finished \
     jobs and keep the pending and failed ones. Inspect the queue first with \
     render_queue_list.",
    RenderQueueClear,
    |py: Python<'_>, p: RenderQueueClear| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let completed_only = p.completed_only.unwrap_or(false);

        // KNOWN ISSUE fixed: the original Python kept only `status ==
        // "pending"` when `completed_only` was set, which also silently
        // discarded every `"failed: ..."` job — contrary to what "clear
        // completed jobs" implies. Fixed to drop exactly the completed jobs,
        // leaving pending and failed jobs untouched.
        let queue: Vec<Value> = if completed_only {
            render_queue_get(&s)?
                .into_iter()
                .filter(|j| j.get("status").and_then(Value::as_str) != Some("completed"))
                .collect()
        } else {
            Vec::new()
        };

        let remaining = queue.len();
        render_queue_set(&s, &queue)?;
        Ok(json!({ "cleared": true, "remaining": remaining }))
    }
);
