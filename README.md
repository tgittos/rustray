# rustray

A work-in-progress ray tracer written while learning Rust.

![](samples/bouncing_spheres.png)

```
Rendering a 1200x675 image with 100 samples per pixel and max depth 50
Rendering Stats:
--------------------------
Total Hits: 206050069
Total Samples: 206050069
Stat: scene_hit
  P50: (250ns, 0ns)
  P90: (334ns, 0ns)
  P99: (458ns, 0ns)

Stat: scene_sample
  P50: (0ns, 0ns)
  P90: (0ns, 0ns)
  P99: (0ns, 0ns)

Stat: lambertian_hit
  P50: (208ns, 0ns)
  P90: (292ns, 0ns)
  P99: (500ns, 0ns)

Stat: lambertian_sample
  P50: (0ns, 42ns)
  P90: (0ns, 1.375µs)
  P99: (0ns, 6.167µs)

Stat: metallic_hit
  P50: (208ns, 0ns)
  P90: (292ns, 0ns)
  P99: (542ns, 0ns)

Stat: metallic_sample
  P50: (0ns, 292ns)
  P90: (0ns, 1.834µs)
  P99: (0ns, 10.919µs)

Stat: dielectric_hit
  P50: (250ns, 0ns)
  P90: (334ns, 0ns)
  P99: (541ns, 0ns)

Stat: dielectric_sample
  P50: (0ns, 375ns)
  P90: (0ns, 1.625µs)
  P99: (0ns, 9.751µs)

Total Hit Time: 0h 0m 46s 457ms
Total Sample Time: 0h 2m 6s 413ms
--------------------------
Overall Total Time: 0h 2m 52s 871ms
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
