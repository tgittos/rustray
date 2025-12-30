use std::env;
use std::path::{Path, PathBuf};
use std::time;

use rustray::core::scene;
use rustray::stats::charts;
use rustray::{raytrace, raytrace_concurrent};

// const SAMPLES: &[u32] = &[10, 50, 100, 200, 500, 1000, 2000, 5000, 10000];
// const SAMPLE_LABELS: &[&str] = &["10", "50", "100", "200", "500", "1k", "2k", "5k", "10k"];
const SAMPLES: &[u32] = &[10, 50, 100, 200, 500, 1000];
const SAMPLE_LABELS: &[&str] = &["10", "50", "100", "200", "500", "1k"];

fn format_duration(dur: time::Duration) -> String {
    let secs = dur.as_secs();
    let millis = dur.subsec_millis();
    format!("{}.{:03} seconds", secs, millis)
}

fn main() {
    let mut rng = rand::rng();
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| String::from("rustray"));
    let scene_path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("scenes/bouncing_spheres.toml"));
    let is_concurrent = args.next().map(|s| s == "--concurrent").unwrap_or(false);

    if !scene_path.is_file() {
        eprintln!(
            "Scene file not found: {}. Usage: {} <scene-file>",
            scene_path.display(),
            program_name
        );
        std::process::exit(1);
    }

    let mut render = match scene::load_from_file(&mut rng, scene_path.as_path()) {
        Ok(result) => result,
        Err(err) => {
            eprintln!(
                "Failed to load scene from {}: {}",
                scene_path.display(),
                err
            );
            std::process::exit(1);
        }
    };

    let mut wall_times = Vec::new();

    for &ns in SAMPLES.iter() {
        render.samples = ns;

        let render_start = time::Instant::now();

        let data = if is_concurrent {
            let cpus = num_cpus::get();
            println!(
                "Rendering a {}x{} image with {} samples per pixel and max depth {} using {} threads",
                render.width,
                render.width as f32 * render.camera.aspect_ratio,
                render.samples,
                render.depth,
                cpus
            );

            raytrace_concurrent(&render)
        } else {
            println!(
                "Rendering a {}x{} image with {} samples per pixel and max depth {}",
                render.width,
                render.width as f32 * render.camera.aspect_ratio,
                render.samples,
                render.depth
            );
            raytrace(&mut rng, &render)
        };

        wall_times.push(render_start.elapsed());

        let filename = if is_concurrent {
            format!(
                "{}_{}spp_concurrent",
                scene_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output"),
                ns
            )
        } else {
            format!(
                "{}_{}spp",
                scene_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output"),
                ns
            )
        };

        match image::save_buffer(
            &Path::new(&format!("samples/{}.png", filename)),
            data.as_slice(),
            render.width,
            (render.width as f32 / render.camera.aspect_ratio) as u32,
            image::ColorType::Rgb8,
        ) {
            Ok(_) => println!("Image saved."),
            Err(e) => eprintln!("Failed to save image: {}", e),
        }
    }

    match charts::chart(
        scene_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output"),
        &SAMPLE_LABELS.to_vec(),
        &wall_times,
        is_concurrent,
    ) {
        Ok(_) => println!("Render profile chart saved."),
        Err(e) => eprintln!("Failed to save render profile chart: {}", e),
    }

    println!("\n=== Render Profile Summary ===");
    for (i, &ns) in SAMPLES.iter().enumerate() {
        println!(
            "{} samples: Render Wall Time: {}",
            ns,
            format_duration(wall_times[i])
        );
    }
}
