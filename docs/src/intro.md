# Introduction

**blender-mcp-rs** lets an AI agent — Claude Code, or any [MCP](https://modelcontextprotocol.io)
client — drive [Blender 5](https://www.blender.org). The agent gets **241 tools**: 233
that act on the scene (objects, meshes, materials, a unified node layer covering all 570
of Blender's node types, rigging, physics, animation, rendering…), 5 that manage the
Blender process itself (launch, status, quit, kill, restart), and 3 that search the
built-in knowledge base (`kdb_search`, `kdb_read`, `kdb_guide`) — those answer with
Blender closed.

Every tool is **written in Rust**. Python survives only where Blender forces it: Blender
loads add-ons as Python modules and only its own Python process can touch `bpy`. That
add-on is deliberately thin — it forwards each command to the Rust code and does nothing
else.

## What you can do with it

- *"Build me a night-time city block with instanced buildings and neon signage, then
  render an aerial view."* — the agent creates geometry, materials, lights and cameras,
  keyframes a flyover, and renders it. (See the [worked example](./example-city.md).)
- *"Make this material look like brushed gold."* — the agent inspects the real node
  sockets, builds a shader graph in one call, renders a preview sphere, and iterates.
- *"Start Blender and set up the scene."* — the agent launches Blender itself; you do not
  have to have it open first.

## Why it is built this way

The design turns on one idea: a tool is **declared once** and that single declaration
serves two processes that never see each other's memory. The result is that the schema
the model reads and the code that runs against `bpy` **cannot drift apart** — it is a
property of the build, not of anyone's discipline. The [whitepaper](./whitepaper.md)
explains how.

## Where to go next

| You want to… | Read |
|---|---|
| Understand the architecture | [Whitepaper](./whitepaper.md) |
| Install it and drive Blender | [User guide](./user-guide.md) |
| Add a tool or hack on it | [Developer guide](./dev-guide.md) |
| Copy node/shader setups | [Recipes](./recipes.md) |
| See a full scene built end to end | [Worked example](./example-city.md) |

> Blender 5.2 LTS is the target. The tools use the Blender 5.2 API; several older APIs were
> removed or renamed and are handled explicitly — see the developer guide.
