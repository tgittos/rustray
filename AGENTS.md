# Repository Guidelines

## Project Structure & Module Organization
- Binary entry point in `src/main.rs`; reusable rendering API in `src/lib.rs` (`raytrace` expects a caller-supplied `&mut rand::rngs::ThreadRng` and prints timing stats).
- Scenes load from TOML files via `core::scene_file` (default `scenes/bouncing_spheres.toml`); `raytrace` takes a `core::render::Render` (width, samples, depth, camera, scene).
- Core plumbing in `src/core/` (camera, ray, bbox, BVH, render container, world sky gradient); scene objects live in `core::object` and `core::scene`.
- Geometry is in `src/geometry/` (primitives: sphere, quad, cube; transforms; instance wrapper applies transforms and motion blur).
- Materials in `src/materials/` (lambertian, metallic, dielectric, diffuse light, instances with optional albedo tint); textures in `src/textures/` (color, checker, Perlin noise, UV image).
- Traits under `src/traits/` (`Hittable`, `Renderable`, `Sampleable`, `Texturable`); math helpers in `src/math/` (vec, mat, interval, Perlin); timing stats in `src/stats.rs`.
- Render output is written to `samples/<scene>.png`; `target/` is build output (do not commit).

## Build, Test, and Development Commands
- `cargo fmt` — format the workspace; run before sending changes.
- `cargo clippy -- -D warnings` — lint and keep the codebase warning-free.
- `cargo build` — compile library and binary.
- `cargo run --release [scenes/bouncing_spheres.toml]` — render a scene to `samples/<scene>.png`; override the scene path with a CLI arg.
- `cargo test` — executes tests when added; currently none exist.

## Coding Style & Naming Conventions
- Rust edition 2024; prefer standard 4-space indentation and rustfmt defaults.
- Types and structs `CamelCase`, functions/modules/files `snake_case`, constants `SCREAMING_SNAKE_CASE`.
- Pair geometry and materials via `core::object::RenderObject` (a `GeometryInstance` + `MaterialInstance`); ensure hittables report accurate bounding boxes (including transformed/moving geometry) so BVH culling stays valid.
- Preserve ray time values when scattering (camera and motion transforms rely on `ray.time`); keep sampling math side-effect free and prefer immutable `Vec3` usage with explicit cloning only when needed.
- Keep timing instrumentation in `raytrace`/materials intact; if adding new hot paths, record comparable hit/sample durations through `stats`.

## Testing Guidelines
- Add unit tests near modules (e.g., `src/math/vec.rs`) and integration tests under `tests/` that exercise ray paths end-to-end.
- Prefer deterministic randomness in tests by seeding an RNG (`StdRng` or similar) when using functions that accept generic `rand::Rng`.
- Cover hit detection edge cases (`t_min`/`t_max`), BVH culling, motion-blurred transforms, refraction/reflectance correctness, and texture sampling (checker, noise, UV).
- Keep tests fast; avoid large renders—use tiny viewports (e.g., 16x8 with few samples).

## Commit & Pull Request Guidelines
- Ensure `cargo fmt` and `cargo clippy -- -D warnings` pass; note test status and reproduction steps in the PR body.
