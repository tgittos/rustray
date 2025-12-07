# rustray

A work-in-progress ray tracer written in while learning Rust.

![](output.png)

```
Rendering a 1200x675 image with 100 samples per pixel and max depth 50
Rendering Stats:
--------------------------
Total Hits: 209332892
Total Samples: 209332892
Stat: scene_hit
  P50: (292ns, 0ns)
  P90: (625ns, 0ns)
  P99: (791ns, 0ns)

Stat: scene_sample
  P50: (0ns, 0ns)
  P90: (0ns, 0ns)
  P99: (0ns, 0ns)

Stat: diffuse_hit
  P50: (209ns, 0ns)
  P90: (333ns, 0ns)
  P99: (625ns, 0ns)

Stat: diffuse_sample
  P50: (0ns, 42ns)
  P90: (0ns, 1.625µs)
  P99: (0ns, 7.043µs)

Stat: metallic_hit
  P50: (208ns, 0ns)
  P90: (334ns, 0ns)
  P99: (750ns, 0ns)

Stat: metallic_sample
  P50: (0ns, 333ns)
  P90: (0ns, 2.042µs)
  P99: (0ns, 11.919µs)

Stat: dielectric_hit
  P50: (292ns, 0ns)
  P90: (542ns, 0ns)
  P99: (917ns, 0ns)

Stat: dielectric_sample
  P50: (0ns, 458ns)
  P90: (0ns, 1.792µs)
  P99: (0ns, 7.627µs)

Total Hit Time: 0h 1m 0s 570ms
Total Sample Time: 0h 2m 23s 501ms
--------------------------
Overall Total Time: 0h 3m 24s 71ms
--------------------------
Image saved to output.png
```

## Quick start
- Install the Rust toolchain (2024 edition).
- Render the default scene (1200px wide, 16:9 aspect, 100 samples per pixel, max depth 50):

```bash
cargo run --release
```

## Project layout
- `src/main.rs` — binary entry point that wires the camera, scene objects (including motion-blurred spheres), and materials together, then writes a PNG.
- `src/lib.rs` — exposes the `raytrace` function used by the binary; accepts a caller-supplied `&mut rand::rngs::ThreadRng` and prints timing stats.
- `src/core/` — math and camera primitives (`vec`, `ray`, `camera`, `interval`, `bbox`, `scene`).
- `src/materials/` — scattering logic for diffuse, metallic, and dielectric surfaces.
- `src/primitives/` — hittable geometry such as spheres (static or moving) and the skybox background.
- `src/traits/` — `Hittable`, `Sampleable`, and `Renderable` traits plus the `RenderableList` glue that pairs geometry with materials.
- `src/utils/stats.rs` — lightweight render timing aggregation.

## Common tasks
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Build: `cargo build`
- Render: `cargo run --release` (produces `output.png`)

## Adjusting renders
- Resolution and sampling: edit `nx`, `ns`, and `max_depth` near the top of `src/main.rs` (height is derived from `nx` and aspect ratio).
- Scene contents: the same file builds the scene by pairing primitives (e.g., `Sphere`, `Skybox`) with materials (`Diffuse`, `Metallic`, `Dielectric`) via `renderable::create_renderable`.
- Camera: tweak `CameraConfig` in `src/main.rs` or construct your own via `Camera::with_config`.

## Extending the renderer
- Add geometry by implementing `Hittable` under `src/primitives/`.
- Add materials by implementing `Sampleable` under `src/materials/`.
- Compose them with `renderable::create_renderable` and add to `Scene` to render new objects.
