# rustray

A small Rust ray tracer with a reusable rendering core (`src/lib.rs`) and a demo binary (`src/main.rs`) that renders a simple scene to `output.ppm`.

## Quick start
- Install the Rust toolchain (2024 edition). No extra crates beyond `rand` are required.
- Render the default scene (200x100, 50 samples per pixel, max depth 8):

```bash
cargo run --release
```

The generated image is written to `output.ppm` in the repo root.

## Project layout
- `src/main.rs` — binary entry point that wires the camera, scene objects, and materials together, then writes a PPM.
- `src/lib.rs` — exposes the `raytrace` function used by the binary; houses the reusable API.
- `src/core/` — math and camera primitives (`vec`, `ray`, `camera`, `scene`).
- `src/materials/` — scattering logic for diffuse, metallic, and dielectric surfaces.
- `src/primitives/` — hittable geometry such as spheres and the skybox.
- `src/traits/` — `Hittable`, `Sampleable`, and `Renderable` traits that pair geometry with materials.
- `src/formats/ppm.rs` — minimal PPM writer used by the demo.

## Common tasks
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Build: `cargo build`
- Tests: `cargo test` (none yet)
- Render: `cargo run --release` (produces `output.ppm`)

## Adjusting renders
- Resolution and sampling: edit `nx`, `ny`, `ns`, and `max_depth` near the top of `src/main.rs`.
- Scene contents: the same file builds the scene by pairing primitives (e.g., `Sphere`, `Skybox`) with materials (`Diffuse`, `Metallic`, `Dielectric`) via `renderable::create_renderable`.
- Camera: tweak `CameraConfig` in `src/main.rs` or construct your own via `Camera::with_config`.

## Extending the renderer
- Add geometry by implementing `Hittable` under `src/primitives/`.
- Add materials by implementing `Sampleable` under `src/materials/`.
- Compose them with `renderable::create_renderable` and add to `Scene` to render new objects.

