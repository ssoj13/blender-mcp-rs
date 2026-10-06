"""datablock.rs coverage: bpy.data.texts, sounds/speakers, movieclips.

Every assertion reads state back off `bpy`, not off a tool's echoed JSON. Media for
sound_load/movieclip_load is generated on disk here rather than assumed to ship:
a tiny valid WAV via the stdlib `wave` module, and a tiny valid PNG via `bpy.data`
itself (Blender loads a single still image as a one-frame movie clip, confirmed live
against Blender 5.0 — no video codec or ffmpeg dependency needed for the test).

Ground truth verified live (see datablock.rs's module doc for the full record):
- `Text.write()` appends, so overwriting needs `clear()` first.
- `bpy.data.sounds.load()` never raises; a failed decode leaves `channels == "INVALID"`.
- `bpy.data.movieclips.load()` raises for a missing path, but not for an existing file
  it cannot decode — that case leaves `frame_duration == 0`.
"""

import struct
import tempfile
import wave
from pathlib import Path

import bpy


def _make_wav(path: Path) -> None:
    with wave.open(str(path), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(8000)
        samples = [int(3000 * ((i % 40) / 40.0)) for i in range(400)]
        w.writeframes(b"".join(struct.pack("<h", s) for s in samples))


def _make_png(path: Path, size: int = 16) -> None:
    img = bpy.data.images.new(f"__mk_{path.stem}", size, size)
    img.filepath_raw = str(path)
    img.file_format = "PNG"
    img.save()
    bpy.data.images.remove(img)


def run(h):
    tmp_dir = Path(tempfile.mkdtemp(prefix="mcp_datablock_test_"))

    # ------------------------------------------------------------------
    # text datablocks (bpy.data.texts) — driver expressions, OSL, scripts
    # ------------------------------------------------------------------
    h.section("text-datablock")

    h.call("text_datablock_create", {"name": "DriverExpr", "body": "sin(frame)"})
    h.check("text_datablock_create: datablock exists with the given body",
            "DriverExpr" in bpy.data.texts
            and bpy.data.texts["DriverExpr"].as_string() == "sin(frame)")

    h.call("text_datablock_create", {"name": "EmptyText"})
    h.check("text_datablock_create with no body: empty buffer",
            bpy.data.texts["EmptyText"].as_string() == "")

    got = h.call("text_datablock_get", {"name": "DriverExpr"})
    h.check("text_datablock_get: returns the real body", got["body"] == "sin(frame)")

    h.call("text_datablock_set", {"name": "DriverExpr", "body": "cos(frame) * 2"})
    h.check("text_datablock_set: body actually replaced, not appended",
            bpy.data.texts["DriverExpr"].as_string() == "cos(frame) * 2")

    listing = h.call("text_datablock_list")
    h.check("text_datablock_list: sees both texts created above",
            {t["name"] for t in listing} >= {"DriverExpr", "EmptyText"})

    h.call("text_datablock_remove", {"name": "EmptyText"})
    h.check("text_datablock_remove: gone from bpy.data.texts",
            "EmptyText" not in bpy.data.texts)

    h.errors("text_datablock_get on missing text errors",
              "text_datablock_get", {"name": "NoSuchText"}, "not found")
    h.errors("text_datablock_set on missing text errors",
              "text_datablock_set", {"name": "NoSuchText", "body": "x"}, "not found")
    h.errors("text_datablock_remove on missing text errors",
              "text_datablock_remove", {"name": "NoSuchText"}, "not found")

    # This project already has text_create/text_update for 3D FONT objects (mesh.rs).
    # Confirm the two namespaces do not collide: the FONT object path is untouched by
    # the datablock tools above.
    h.call("text_create", {"name": "TextObj3D", "body": "Hello"})
    h.check("text_create (3D object) unaffected: still exists, not in bpy.data.texts "
            "as a *_datablock entry",
            "TextObj3D" in bpy.data.objects and bpy.data.objects["TextObj3D"].type == "FONT")

    # ------------------------------------------------------------------
    # sound / speaker
    # ------------------------------------------------------------------
    h.section("sound-speaker")

    wav_path = tmp_dir / "tone.wav"
    _make_wav(wav_path)

    loaded = h.call("sound_load", {"path": str(wav_path), "name": "ToneSound"})
    h.check("sound_load: real Sound datablock, channels resolved (not INVALID)",
            "ToneSound" in bpy.data.sounds
            and bpy.data.sounds["ToneSound"].channels != "INVALID"
            and bpy.data.sounds["ToneSound"].samplerate == 8000)
    h.check("sound_load: reported filepath matches the file on disk",
            loaded["filepath"] == str(wav_path))

    slist = h.call("sound_list")
    h.check("sound_list: sees the loaded sound", any(s["name"] == "ToneSound" for s in slist))

    h.errors("sound_load on a nonexistent/undecodable file errors",
              "sound_load", {"path": str(tmp_dir / "does_not_exist.wav")}, "loaded no audio")

    garbage_wav = tmp_dir / "garbage.wav"
    garbage_wav.write_text("not really a wav file, just padding text here")
    h.errors("sound_load on a file that fails to decode errors",
              "sound_load", {"path": str(garbage_wav)}, "loaded no audio")
    h.check("sound_load guard: the failed load did not leave a stray datablock behind",
            not any(s.name.startswith("garbage") for s in bpy.data.sounds))

    added = h.call("speaker_add", {"name": "Spk1", "location": [1.0, 2.0, 3.0], "sound": "ToneSound"})
    h.check("speaker_add: SPEAKER object exists at the requested location",
            "Spk1" in bpy.data.objects
            and bpy.data.objects["Spk1"].type == "SPEAKER"
            and tuple(round(c, 4) for c in bpy.data.objects["Spk1"].location) == (1.0, 2.0, 3.0))
    h.check("speaker_add: sound assigned on creation",
            bpy.data.objects["Spk1"].data.sound is not None
            and bpy.data.objects["Spk1"].data.sound.name == "ToneSound")
    h.check("speaker_add: return payload reports the assigned sound", added["sound"] == "ToneSound")

    h.call("speaker_add", {"name": "Spk2"})
    h.check("speaker_add with no sound: created silent (no crash, sound is None)",
            "Spk2" in bpy.data.objects and bpy.data.objects["Spk2"].data.sound is None)

    h.call("speaker_set", {
        "name": "Spk2", "volume": 0.25, "pitch": 1.5, "attenuation": 2.0,
        "distance_max": 50.0, "distance_reference": 5.0,
        "cone_angle_outer": 90.0, "cone_angle_inner": 30.0, "cone_volume_outer": 0.1,
        "muted": True, "sound": "ToneSound",
    })
    spk2 = bpy.data.objects["Spk2"].data
    h.check("speaker_set: every property actually landed on Speaker data",
            abs(spk2.volume - 0.25) < 1e-6 and abs(spk2.pitch - 1.5) < 1e-6
            and abs(spk2.attenuation - 2.0) < 1e-6 and abs(spk2.distance_max - 50.0) < 1e-6
            and abs(spk2.distance_reference - 5.0) < 1e-6
            and abs(spk2.cone_angle_outer - 90.0) < 1e-6
            and abs(spk2.cone_angle_inner - 30.0) < 1e-6
            and abs(spk2.cone_volume_outer - 0.1) < 1e-6
            and spk2.muted is True and spk2.sound is not None and spk2.sound.name == "ToneSound")

    h.call("speaker_set", {"name": "Spk2", "sound": ""})
    h.check("speaker_set with empty sound: unassigned", bpy.data.objects["Spk2"].data.sound is None)

    speakers = h.call("speaker_list")
    h.check("speaker_list: sees both speakers created above",
            {s["name"] for s in speakers} >= {"Spk1", "Spk2"})

    h.errors("speaker_add with an unknown sound name errors, no orphan object left",
              "speaker_add", {"name": "SpkBad", "sound": "NoSuchSound"}, "sound not found")
    h.check("speaker_add guard: failed sound lookup created no object",
            "SpkBad" not in bpy.data.objects)
    h.errors("speaker_set on a non-SPEAKER object errors",
              "speaker_set", {"name": "TextObj3D", "volume": 0.5}, "not a SPEAKER")
    h.errors("speaker_set on a missing object errors",
              "speaker_set", {"name": "NoSuchObj", "volume": 0.5}, "not found")

    # ------------------------------------------------------------------
    # movie clips
    # ------------------------------------------------------------------
    h.section("movieclip")

    png_path = tmp_dir / "footage.png"
    _make_png(png_path, size=64)

    clip = h.call("movieclip_load", {"path": str(png_path), "name": "Footage"})
    h.check("movieclip_load: real MovieClip datablock with the right filepath/size",
            "Footage" in bpy.data.movieclips
            and bpy.data.movieclips["Footage"].filepath == str(png_path)
            and tuple(bpy.data.movieclips["Footage"].size) == (64, 64)
            and bpy.data.movieclips["Footage"].frame_duration >= 1)
    h.check("movieclip_load: reported size matches ground truth", clip["size"] == [64, 64])

    clips = h.call("movieclip_list")
    h.check("movieclip_list: sees the loaded clip", any(c["name"] == "Footage" for c in clips))

    h.call("movieclip_set", {"name": "Footage", "frame_start": 5, "frame_offset": 2,
                              "colorspace": "Non-Color"})
    fc = bpy.data.movieclips["Footage"]
    h.check("movieclip_set: frame_start/frame_offset/colorspace actually changed",
            fc.frame_start == 5 and fc.frame_offset == 2
            and fc.colorspace_settings.name == "Non-Color")

    h.errors("movieclip_load on a nonexistent path errors",
              "movieclip_load", {"path": str(tmp_dir / "ghost.mp4")}, "cannot load")

    garbage_clip = tmp_dir / "garbage_footage.mp4"
    garbage_clip.write_bytes(b"not a real video container" * 20)
    h.errors("movieclip_load on an undecodable existing file errors",
              "movieclip_load", {"path": str(garbage_clip)}, "loaded no frames")
    h.check("movieclip_load guard: the failed load did not leave a stray datablock behind",
            not any(c.name.startswith("garbage_footage") for c in bpy.data.movieclips))

    h.errors("movieclip_set on a missing clip errors",
              "movieclip_set", {"name": "NoSuchClip", "frame_start": 1}, "not found")
