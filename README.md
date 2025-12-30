# rustray

A work-in-progress ray tracer following the ["Ray Tracing in One Weekend"](https://raytracing.github.io/) series. Scenes are authored in TOML, loaded into `core::render::Render`, and rendered to PNGs under `samples/`. The library exposes both single-threaded and Rayon-powered renderers plus a profiler that charts wall time across SPP sweeps.

## Render a scene
- Install the Rust 2024 toolchain.
- Render a TOML scene (defaults to `scenes/bouncing_spheres.toml`, writes `samples/<scene>.png`):

```bash
cargo run --release --bin rustray -- [path/to/scene.toml] [--concurrent]
```

- Omit the path to use the default scene. Pass `--concurrent` to split the image into row chunks per CPU and render in parallel; the default mode runs the single-threaded `raytrace`.

## Profile rendering
- Sweep through several sample-per-pixel counts and generate a timing chart:

```bash
cargo run --release --bin rustray_profile -- [path/to/scene.toml] [--concurrent]
```

- The profiler renders each configured SPP in `src/bin/rustray_profile.rs` (defaults: 10, 50, 100, 200, 500, and 1000), saving `samples/<scene>_<spp>spp[_concurrent].png`, printing a wall-time summary, and writing `profile/profile_<scene>[_concurrent].png` using `charming`.

## Scene format
- Scenes round-trip through `core::scene_file::{load_render, save_render}`. The TOML schema includes:
  - Global `width`, `samples`, `depth`, and a serialized `camera` (full `Camera` state: origin, lower_left_corner, horizontal/vertical, basis vectors `u`/`v`/`w`, `up`, aperture, focal length, aspect ratio, and vertical FOV). Rays carry a random `time` value to support motion blur.
  - `geometries`: tagged entries for `Sphere`, `Quad`, `Cube` (assembled from quads), or `World` (sky gradient).
  - `materials`: tagged entries for `Lambertian`/`Metallic`/`Dielectric`/`DiffuseLight`/`Isotropic`/`World`, with textures `Color`, `Checker`, `Noise`, or `Uv` (uses assets like `assets/earth.jpg`).
  - `objects`: pairs a geometry id with a material id plus optional `transforms` (`Rotate`, `Translate`, `Scale`, `Move` with time range for motion blur) and an optional `albedo` tint applied by `MaterialInstance`.
  - `volumes`: participating media; references a boundary geometry, phase-function material, density, and optional `boundary_transforms`.
- Scenes are deduped when serialized, so reused geometry/materials stay shared.

## Project layout
- `src/bin/rustray.rs` — CLI renderer that loads a TOML scene, optionally runs `raytrace_concurrent`, and writes `samples/<scene>.png`.
- `src/bin/rustray_profile.rs` — profiling helper that renders multiple SPPs and emits a timing bar chart.
- `src/lib.rs` — exposes `raytrace` (single-threaded) and `raytrace_concurrent` (Rayon) plus helpers for chunking and assembling scanlines.
- `src/core/` — camera/ray/bbox primitives, BVH (`bvh`), threaded chunker (`acceleration`), render container (`render`), renderables/objects (`object`), volumes (`volume`), sky gradient (`world`), and TOML scene loader/saver (`scene_file`).
- `src/geometry/` — hittables (sphere, quad, cube), transforms (rotate/translate/scale/move), and `GeometryInstance` that applies transforms and motion blur-aware bounds.
- `src/materials/` — lambertian, metallic, dielectric, diffuse light, and `MaterialInstance` for optional albedo tinting; `core::volume::Isotropic` provides the volume phase function; `src/textures/` covers color/checker/Perlin noise/UV textures.
- `src/stats/` — chart rendering via `charming` for profiling.
- `examples/` — programmatic scene builders that mirror the TOML files.
- `samples/` holds rendered outputs; `profile/` holds timing charts; `target/` is build output (do not commit).

## Rendering details
- Samples per pixel are snapped to a perfect square for stratified jitter (`sqrt(spp) x sqrt(spp)` grid). Gamma correction is applied via square root before saving.
- BVH culling (built in `Scene::build_bvh`) sits in front of per-object hit tests; every hittable supplies a bounding box, including transformed/moving instances.
- Rays keep their `time` through scattering to keep motion blur and animated transforms consistent.
- Volumes implement an isotropic phase function; the world background is modeled as a `World` hittable/material pair.

## Common tasks
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Build: `cargo build`
- Test: `cargo test` (no tests yet)

## Sample renders

### Final scene from "Ray Tracing in One Weekend: The Next Week"

400 boxes, 1,000+ spheres, 1 diffuse light, a low density volume over the entire scene, glass/metal/diffuse/perlin noise/UV mapping materials, movement transformations

[examples/next_week_scene.rs](examples/next_week_scene.rs)<br />
[scenes/next_week_scene.toml](scenes/next_week_scene.toml)

```
Rendering a 800x800 image with 10000 samples per pixel and max depth 50 using 10 threads
Wall time: 2h 37m 36s 801ms
Image saved to samples/next_week_scene.png
```

![](samples/next_week_scene.png)

### Final scene from "Ray Tracing in One Weekend" 

- 400+ spheres, glass/metal/diffuse materials

[examples/bouncing_spheres.rs](examples/bouncing_spheres.rs)<br />
[scenes/bouncing_spheres.toml](scenes/bouncing_spheres.toml)

```
Rendering a 800x450 image with 10000 samples per pixel and max depth 50 using 10 threads
Wall time: 0h 29m 7s 372ms
Image saved to samples/bouncing_spheres.png
```

![](samples/bouncing_spheres.png)

### Cornell Box

[examples/cornell_box.rs](examples/cornell_box.rs)<br />
[scenes/cornell_box.toml](scenes/cornell_box.toml)

```
Rendering a 600x600 image with 10000 samples per pixel and max depth 50 using 10 threads
Wall time: 0h 22m 57s 831ms
Image saved to samples/cornell_box.png
```

![](samples/cornell_box.png)

## Performance

Concurrent rendering is more efficient on multi-core machines vs. single-threaded rendering, however in both methods the render time grows with samples per pixel, max depth, and scene complexity (number of objects, types of materials, etc).

Melt your CPU rendering pretty things.
![](assets/melt.png)

### Concurrent (Rayon, CPU count)

Profiling render [scenes/bouncing_spheres.toml](scenes/bouncing_spheres.toml) at 10, 50, 100, 200, 500, and 1000 samples-per-pixel.

![](profile/profile_bouncing_spheres_concurrent.png)

### Single-threaded

Profiling render [scenes/bouncing_spheres.toml](scenes/bouncing_spheres.toml) at 10, 50, 100, 200, 500, and 1000 samples-per-pixel.

![](profile/profile_bouncing_spheres.png)
