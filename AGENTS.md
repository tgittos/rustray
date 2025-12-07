# Repository Guidelines

## Project Structure & Module Organization
- Binary entry point in `src/main.rs`; reusable rendering API in `src/lib.rs` (`raytrace` expects a caller-supplied `&mut rand::rngs::ThreadRng` and prints timing stats).
- Core math and camera primitives live under `src/core/` (`vec`, `ray`, `camera`, `interval`, `bbox`, `scene`); hittables must return bounding boxes so `RenderableList` culling stays correct.
- Materials implementing `Sampleable` are in `src/materials/` (diffuse, metallic, dielectric), and hittable geometry lives in `src/primitives/` (sphere with motion blur support, skybox background).
- Rendering traits and glue live under `src/traits/` (`Hittable`, `Sampleable`, `Renderable`, and `RenderableList`), while utility timing helpers are in `src/utils/stats.rs`.
- Rendered output defaults to `output.png` in the repo root via the `image` crate; `target/` is build output (do not commit).

## Build, Test, and Development Commands
- `cargo fmt` — format the workspace; run before sending changes.
- `cargo clippy -- -D warnings` — lint and keep the codebase warning-free.
- `cargo build` — compile library and binary.
- `cargo run --release` — render the default scene (~1200px wide, 16:9 AR, 100 spp, max depth 50) to `output.png`; tweak `nx`, `ns`, and `max_depth` near the top of `src/main.rs`.
- `cargo test` — executes tests when added; currently none exist.

## Coding Style & Naming Conventions
- Rust edition 2024; prefer standard 4-space indentation and rustfmt defaults.
- Types and structs `CamelCase`, functions/modules/files `snake_case`, constants `SCREAMING_SNAKE_CASE`.
- Compose renderables by pairing a `Hittable` with a `Sampleable` via `renderable::create_renderable`; ensure new hittables report accurate bounding boxes (including animated geometry) so scene culling remains valid.
- Preserve ray time values when scattering (camera and spheres use `ray.time` for motion blur); keep sampling math side-effect free and prefer immutable `Vec3` usage with explicit cloning only when needed.
- Keep the timing instrumentation in `raytrace`/materials intact; if adding new hot paths, record comparable hit/sample durations through `utils::stats`.

## Testing Guidelines
- Add unit tests near modules (e.g., `src/core/vec.rs`) and integration tests under `tests/` that exercise ray paths end-to-end.
- Prefer deterministic randomness in tests by seeding an RNG (`StdRng` or similar) when using functions that accept generic `rand::Rng`.
- Cover hit detection edge cases (`t_min`/`t_max`), bounding-box culling, motion-blurred sphere centers, reflection/refraction correctness, and skybox gradient sampling.
- Keep tests fast; avoid large renders—use tiny viewports (e.g., 16x8 with few samples).

## Commit & Pull Request Guidelines
- Ensure `cargo fmt` and `cargo clippy -- -D warnings` pass; note test status and reproduction steps in the PR body.
