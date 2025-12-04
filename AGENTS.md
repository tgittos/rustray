# Repository Guidelines

## Project Structure & Module Organization
- Binary entry point in `src/main.rs`; reusable ray tracer API in `src/lib.rs`.
- Core math and camera primitives live under `src/core` (`vec.rs`, `ray.rs`, `camera.rs`, `scene.rs`).
- Materials and scattering logic are in `src/materials` (`diffuse`, `metallic`, `dielectric`), while hittable geometry is under `src/primitives` (`sphere`, `skybox`).
- Rendering traits sit in `src/traits`; image output helpers are in `src/formats/ppm.rs`.
- Rendered output defaults to `output.ppm` in the repo root; `target/` is build output (do not commit).

## Build, Test, and Development Commands
- `cargo fmt` — format the workspace; run before sending changes.
- `cargo clippy -- -D warnings` — lint and keep the codebase warning-free.
- `cargo build` — check that library and binary compile (debug).
- `cargo run --release` — render the default scene to `output.ppm` (200x100, 50 spp). Increase image size or samples by editing `src/main.rs`.
- `cargo test` — executes tests when added; currently none exist.

## Coding Style & Naming Conventions
- Rust edition 2024; prefer standard 4-space indentation and rustfmt defaults.
- Types and structs `CamelCase`, functions/modules/files `snake_case`, constants `SCREAMING_SNAKE_CASE`.
- Keep renderers small and composable: geometry implements `Hittable`, materials implement `Sampleable`, and `Renderable` pairs the two.
- Favor immutable data and explicit cloning of `Vec3` only where needed; keep ray-sampling math side-effect free.

## Testing Guidelines
- Add unit tests near modules (e.g., `src/core/vec.rs`) and integration tests under `tests/` that exercise full ray paths.
- Prefer deterministic randomness in tests by injecting a seeded RNG instead of `rand::rng()`.
- Cover hit detection edge cases (t_min/t_max), reflection/refraction correctness, and skybox sampling gradients.
- Keep tests fast; avoid large images—use tiny viewports (e.g., 16x8, few samples).

## Commit & Pull Request Guidelines
- Ensure `cargo fmt` and `cargo clippy -- -D warnings` pass; note test status and reproduction steps in the PR body.
