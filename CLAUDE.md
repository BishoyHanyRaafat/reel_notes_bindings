# video-engine — Claude Code Context

## What this project is

An AI-powered educational video generator written in Rust.
An LLM produces a structured JSON lesson script. This engine converts it into a
narrated, illustrated MP4 video with word-synchronised text, animations, and
Skia-rendered frames piped through ffmpeg.

---

## Workspace layout

```
video-engine/
├── Cargo.toml                     ← workspace root
├── CLAUDE.md                      ← you are here
├── examples/
│   └── lesson_derivatives.json    ← test input
└── crates/
    ├── cli/              ← CLI implementation
    │   └── src/
    │       ├── main.rs                    ← CLI entry point (clap)
    │       ├── error.rs                   ← PipelineError enum
    │       └── pipeline/
    │           ├── mod.rs
    │           ├── tts_pass.rs            ← concurrent ElevenLabs TTS
    │           ├── timeline.rs            ← audio duration → frame ranges
    │           └── render_pass.rs         ← frame loop → ffmpeg pipe
    ├── video-schema/              ← pure types, no I/O, no logic
    │   └── src/
    │       ├── lib.rs
    │       ├── root.rs            ← LessonScript, OutputConfig, BackgroundDef
    │       ├── segment.rs         ← SegmentDef, SegmentKind, SegmentControl
    │       ├── text.rs            ← TextDef, FontDef, KeyTerm, Position2D
    │       ├── audio.rs           ← AudioTrack, BackgroundMusic, SoundEffect
    │       └── elevenlabs.rs      ← ElevenLabsConfig, ElevenLabsModel
    ├── tts-client/                ← ElevenLabs HTTP client
    │   └── src/
    │       ├── lib.rs
    │       ├── client.rs          ← TtsClient, synthesise(), word timestamp parsing
    │       └── types.rs           ← TtsResult, WordTimestamp, TtsError
    ├── renderer/                  ← Skia CPU-raster renderer
    │   └── src/
    │       ├── lib.rs
    │       ├── context.rs         ← RenderContext, FrameState, RgbFrame
    │       ├── background.rs      ← draw_background (solid, gradient, image stub)
    │       └── text.rs            ← draw_text_layer, word reveal, key term highlight
    └── encoder/                   ← ffmpeg process wrapper
        └── src/
            ├── lib.rs
            └── ffmpeg.rs          ← FfmpegEncoder, two-pass encode + audio mux
```

---

## Full async pipeline

```
JSON file
  │
  ▼
[1] Load + validate          (main.rs → validate_script)
  │   serde_json → LessonScript
  │   checks: segments present, voice configured
  │
  ▼
[2] Image generation pass    (NOT YET IMPLEMENTED — stub)
  │   resolve image_prompts → generate images → write to asset cache
  │   join_all concurrent
  │
  ▼
[3] TTS pass                 (pipeline/tts_pass.rs)
  │   per active segment: text → ElevenLabs → MP3 + WordTimestamps
  │   join_all concurrent — all segments hit ElevenLabs in parallel
  │   output: Vec<SegmentAudio>
  │
  ▼
[4] Timeline builder         (pipeline/timeline.rs)
  │   audio duration + post_hold + delay → start_frame / end_frame per segment
  │   word timestamps (seconds) → word reveal frame numbers
  │   output: Vec<TimedSegment>
  │
  ▼
[5] Render + encode          (pipeline/render_pass.rs)
  │   for frame 0..total_frames:
  │     find active segment
  │     compute anim_progress (fade in/out)
  │     compute revealed_word_count from word_frames
  │     RenderContext::draw_frame(FrameState) → RgbFrame (raw RGB24)
  │     FfmpegEncoder::write_frame(rgb)
  │   finalise: close stdin → wait → mux audio → final MP4
  │
  ▼
output.mp4
```

---

## Key types (data flow through the pipeline)

| Type | Crate | Role |
|---|---|---|
| `LessonScript` | video-schema | Root deserialized document |
| `SegmentDef` | video-schema | One segment as the LLM wrote it |
| `ElevenLabsConfig` | video-schema | TTS voice + model settings |
| `TtsResult` | tts-client | Audio path + duration + word timestamps |
| `SegmentAudio` | pipeline | `SegmentDef` + its `TtsResult` |
| `TimedSegment` | pipeline | `SegmentDef` + start/end frames + word frames |
| `FrameState<'_>` | renderer | Everything needed to draw one frame |
| `RgbFrame` | renderer | Raw RGB24 pixel buffer for one frame |
| `FfmpegEncoder` | encoder | Owns the ffmpeg child process |

---

## Environment variables

| Variable | Required | Description |
|---|---|---|
| `ELEVENLABS_API_KEY` | Yes | ElevenLabs API key |
| `RUST_LOG` | No | Log level e.g. `video_engine=debug` |

---

## CLI usage

```bash
# Standard render
video-engine --input lesson.json --output lesson.mp4

# Custom work dir (where audio + concat lists go)
video-engine --input lesson.json --output lesson.mp4 --work-dir ./cache

# Fast re-render: skip TTS, reuse cached audio in --work-dir
video-engine --input lesson.json --output lesson.mp4 --skip-tts
```

---

## Build notes

- **First build is slow** — `skia-safe` downloads and compiles Skia C++ (~5–10 min).
  Set `SKIA_LIBRARY_DIR` to a pre-built Skia to avoid this on CI.
- Cross-platform (Linux / macOS / Windows) — no platform-specific code.
- Requires `ffmpeg` on PATH at runtime.
- Requires `ELEVENLABS_API_KEY` env var at runtime.

```bash
cargo build --release
```

---

## Known stubs / TODOs (in priority order)

### 1. Image background in `renderer/src/background.rs`

`BackgroundDef::Image` currently falls back to solid black.
Implement: load image with `skia_safe::Image::from_encoded`, scale to canvas,
apply overlay.

### 3. Image generation pass in `main.rs`

Step 2 of the pipeline is a no-op.
Implement: walk all segments, collect `image_prompt` fields (not yet in schema
for v1), call image gen API (Flux / DALL-E 3 / SD), write to asset cache,
patch `images[]` src to resolved paths.

### 4. Math rendering (`segment.math`)

Not in the v1 schema surface but designed for.
Options: `typst` CLI → SVG → load as Skia image, or `latex` → DVI → PNG.
Recommended: `typst` (pure Rust, faster, no LaTeX install required).

### 5. Code block rendering (`segment.code`)

Planned schema exists (see design docs). Renderer stub needed.
Use `syntect` for syntax highlighting → coloured spans → Skia text runs.

### 6. Crossfade transition between segments

`transition_to_next.type = crossfade` is parsed but not rendered.
In `render_pass.rs`, detect when `frame_num` falls inside the transition window
between two segments and alpha-blend both segments' `draw_frame` outputs.

### 7. Ken Burns on images

`images[].animation.type = ken_burns` is parsed but not rendered.
In the image draw layer, compute `scale = 1.0 + (0.04 * progress)` and offset
to keep the image centred while zooming.

---

## Design rules (do not break these)

- **`video-schema` has zero logic.** No methods that do I/O, no tokio, no reqwest.
  It is types + serde only. Utility methods like `Position2D::resolve()` are fine.

- **`renderer` does not know about time.** It receives `FrameState` and draws.
  It never reads audio files or calls ElevenLabs.

- **`encoder` does not know about segments.** It receives raw RGB bytes and paths.
  All segment logic lives in `pipeline/`.

- **`tts-client` is stateless per call.** `TtsClient` holds only the HTTP client
  and API key. No caching, no segment awareness.

- **All I/O in the pipeline is async.** No `std::fs::read` in pipeline code —
  use `tokio::fs`. The renderer and encoder use sync I/O internally where needed
  (Skia is sync; ffmpeg stdin write is async via `tokio::process`).

- **Segment IDs are stable.** Never renumber. Use `control.enabled = false` or
  `control.skip = true` to exclude a segment. This keeps LLM JSON diffs clean.

---

## Adding a new element type (e.g. `segment.table`)

1. Add the type to `crates/video-schema/src/` (new file or extend `segment.rs`).
2. Add `pub table: Option<TableDef>` to `SegmentDef`.
3. Add `draw_table()` to `crates/renderer/src/` (new file `table.rs`).
4. Call `draw_table()` from `context.rs → RenderContext::draw_frame()` after text.
5. No changes needed to `tts-client`, `encoder`, or `pipeline/` unless the
   element affects timing (it won't for a static table).

---

## Segment JSON quick reference

```jsonc
{
  "id": 1,                         // stable, never renumber
  "type": "speech",                // speech | pause | transition | title | chapter | outro
  "mood": "analytical",            // neutral | analytical | excited | serious | warm | tense
  "pacing": "normal",              // slow | normal | fast

  "control": {
    "enabled": true,               // false = excluded from render entirely
    "skip": false,                 // true = intentional short-form omission
    "speed_multiplier": 1.0,       // 0.5 = half speed, 2.0 = double
    "delay_before_ms": 0,
    "delay_after_ms": 0
  },

  "text": {
    "content": "...",
    "key_terms": [{ "word": "...", "color": "#hex", "bold": true }],
    "reveal": "word_by_word",      // all_at_once | word_by_word | line_by_line | typewriter
    "reveal_sync": "audio",        // audio (ElevenLabs timestamps) | timed
    "font": { "size": 64, "weight": "semibold" },
    "color": "#ffffff",
    "align": "center",
    "position": { "x": "center", "y": "center" },
    "max_width": 0.8
  },

  "elevenlabs": {                  // overrides defaults.elevenlabs for this segment
    "voice_id": "...",
    "stability": 0.4,
    "style": 0.5,
    "speed": 0.93
  },

  "background": { "type": "solid", "color": "#1a1a2e" },  // segment-level override

  "animation_in":  { "type": "fade", "duration_ms": 400, "easing": "ease_out" },
  "animation_out": { "type": "fade", "duration_ms": 300, "easing": "ease_in"  },
  "post_hold_seconds": 0.8,

  "transition_to_next": {
    "type": "crossfade",           // cut | fade | crossfade | wipe_left | wipe_right | ...
    "duration_ms": 500
  }
}
```
