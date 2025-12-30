# Repository Guidelines

## Project Structure & Module Organization
- Binaries live in `src/bin/`: `rustray.rs` loads a TOML scene and renders it; `rustray_profile.rs` sweeps SPP counts and writes timing charts. `src/lib.rs` exposes `raytrace` (single-threaded, requires `&mut rand::rngs::ThreadRng`) and `raytrace_concurrent` (Rayon).
- Scenes load from TOML via `core::scene_file` (default `scenes/bouncing_spheres.toml`); `core::render::Render` bundles width, samples, depth, camera, and scene. `scene_file` also supports saving a render back to TOML, deduping shared geometries/materials.
- Core plumbing in `src/core/` (camera, ray, bbox, BVH, render container, renderables/object wiring, world sky gradient, volumes, plus `acceleration` for threaded chunking). Scene objects live in `core::object` and `core::scene` with optional BVH acceleration.
- Geometry lives in `src/geometry/` (sphere, quad, cube assembled from quads; transforms include rotate/translate/scale/move for motion blur; `GeometryInstance` applies transforms and propagates bounding boxes).
- Materials in `src/materials/` (lambertian, metallic, dielectric, diffuse light, `MaterialInstance` for optional albedo tint); `core::volume::Isotropic` provides the volume phase function; textures in `src/textures/` (color, checker, Perlin noise, UV image backed by assets like `assets/earth.jpg`).
- Traits under `src/traits/` (`Hittable`, `Renderable`, `Scatterable`, `Texturable`); math helpers in `src/math/` (vec, mat, onb, interval, pdf, Perlin); chart rendering lives in `src/stats/`.
- Render output is written to `samples/<scene>.png` (and `samples/<scene>_<spp>spp[_concurrent].png` for profiling); profiling charts land in `profile/`; `target/` is build output (do not commit).

## Build, Test, and Development Commands
- `cargo fmt` — format the workspace; run before sending changes.
- `cargo clippy -- -D warnings` — lint and keep the codebase warning-free.
- `cargo build` — compile library and binaries.
- `cargo run --release --bin rustray -- [scenes/bouncing_spheres.toml] [--concurrent]` — render a scene to `samples/<scene>.png`; omit the path for the default scene and add `--concurrent` for Rayon-based chunking.
- `cargo run --release --bin rustray_profile -- [scenes/bouncing_spheres.toml] [--concurrent]` — profile multiple SPP settings, emit a wall-time summary, and write `profile/profile_<scene>[_concurrent].png`.
- `cargo test` — executes tests when added; currently none exist.

## Coding Style & Naming Conventions
- Rust edition 2024; prefer standard 4-space indentation and rustfmt defaults.
- Types and structs `CamelCase`, functions/modules/files `snake_case`, constants `SCREAMING_SNAKE_CASE`.
- Pair geometry and materials via `core::object::RenderObject` (a `GeometryInstance` + `MaterialInstance`); ensure hittables report accurate bounding boxes (including transformed/moving geometry) so BVH culling stays valid.
- Preserve ray time values when scattering (camera and motion transforms rely on `ray.time`); keep sampling math side-effect free and prefer immutable `Vec3` usage with explicit cloning only when needed.
- Timing output today is limited to wall-time logging in `raytrace`/`raytrace_concurrent` and profiling charts under `src/stats/`; keep output paths consistent if you add new metrics.

## Testing Guidelines
- Add unit tests near modules (e.g., `src/math/vec.rs`) and integration tests under `tests/` that exercise ray paths end-to-end.
- Prefer deterministic randomness in tests by seeding an RNG (`StdRng` or similar) when using functions that accept generic `rand::Rng`.
- Cover hit detection edge cases (`t_min`/`t_max`), BVH culling, motion-blurred transforms, participating media, refraction/reflectance correctness, and texture sampling (checker, noise, UV).
- Keep tests fast; avoid large renders—use tiny viewports (e.g., 16x8 with few samples).

## Commit & Pull Request Guidelines
- Ensure `cargo fmt` and `cargo clippy -- -D warnings` pass; note test status and reproduction steps in the PR body.
