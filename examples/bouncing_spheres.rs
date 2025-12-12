use rand::Rng;
use std::{path::Path, sync::Arc};

use rustray::core::{camera, object, render, scene, scene_file, world};
use rustray::geometry::{instance::GeometryInstance, primitives::sphere, transform};
use rustray::materials::{dielectric, instance::MaterialInstance, lambertian, metallic};
use rustray::math::vec;
use rustray::textures::{checker, color};

use rustray::raytrace;

fn main() {
    let mut rng = rand::rng();

    let nx = 1200;
    let ar = 16.0 / 9.0;
    let ny = (nx as f32 / ar) as u32;
    let ns = 100;
    let max_depth = 50;

    // scene setup
    let camera_config = camera::CameraConfig {
        origin: vec::Vec3::new(13.0, 2.0, 3.0),
        look_at: vec::Vec3::new(0.0, 0.0, 0.0),
        up: vec::Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio: ar,
        viewport_height: 2.0,
        focal_length: 10.0,
        aperture: 0.1,
        vertical_fov: 20.0,
    };
    let camera = camera::Camera::with_config(camera_config);
    let mut scene = scene::Scene::new();

    let static_sphere_template = Arc::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, 0.0), 0.2));
    let large_sphere_template = Arc::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, 0.0), 1.0));
    let ground_sphere_template =
        Arc::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, 0.0), 1000.0));

    let diffuse_base = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(1.0, 1.0, 1.0)),
    )));
    let diffuse_template = || MaterialInstance::new(diffuse_base.clone());

    let metal_template = |roughness: f32| {
        MaterialInstance::new(Arc::new(metallic::Metallic::new(
            &vec::Vec3::new(1.0, 1.0, 1.0),
            roughness,
        )))
    };

    let dielectric_glass = Arc::new(dielectric::Dielectric::new(1.5));

    for i in -11..11 {
        for j in -11..11 {
            let choose_moving: bool = rng.random::<f32>() < 0.5;
            let choose_mat: f32 = rng.random::<f32>();
            let center = vec::Vec3::new(
                i as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                j as f32 + 0.9 * rng.random::<f32>(),
            );

            if (center - vec::Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: MaterialInstance;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec::random(&mut rng) * vec::random(&mut rng);
                    sphere_material = diffuse_template().with_albedo(albedo);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec::random(&mut rng) * vec::random(&mut rng);
                    let fuzz = rng.random::<f32>() * 0.5;
                    sphere_material = metal_template(fuzz).with_albedo(albedo);
                } else {
                    // glass
                    sphere_material = MaterialInstance::new(dielectric_glass.clone());
                }

                let mut geometry_instance = GeometryInstance::new(static_sphere_template.clone());
                if choose_moving {
                    let motion = 0.5 * rng.random::<f32>();
                    geometry_instance
                        .transforms
                        .push(transform::Transform::Move {
                            start: vec::Vec3::new(0.0, 0.0, 0.0),
                            end: vec::Vec3::new(0.0, motion, 0.0),
                            time_start: 0.0,
                            time_end: 1.0,
                        });
                }
                geometry_instance
                    .transforms
                    .push(transform::Transform::Translate(center));

                scene.add_object(Box::new(object::RenderObject {
                    geometry_instance,
                    material_instance: sphere_material,
                }));
            }
        }
    }

    let mut center_sphere_geometry = GeometryInstance::new(large_sphere_template.clone());
    center_sphere_geometry
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            0.0, 1.0, 0.0,
        )));
    let center_sphere = object::RenderObject {
        geometry_instance: center_sphere_geometry,
        material_instance: MaterialInstance::new(dielectric_glass.clone()),
    };

    let mut left_sphere_geometry = GeometryInstance::new(large_sphere_template.clone());
    left_sphere_geometry
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            -4.0, 1.0, 0.0,
        )));
    let left_sphere = object::RenderObject {
        geometry_instance: left_sphere_geometry,
        material_instance: MaterialInstance::new(Arc::new(lambertian::Lambertian::new(Box::new(
            color::ColorTexture::new(vec::Vec3::new(0.4, 0.2, 0.1)),
        )))),
    };

    let mut right_sphere_geometry = GeometryInstance::new(large_sphere_template.clone());
    right_sphere_geometry
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            4.0, 1.0, 0.0,
        )));
    let right_sphere = object::RenderObject {
        geometry_instance: right_sphere_geometry,
        material_instance: metal_template(0.0).with_albedo(vec::Vec3::new(0.7, 0.6, 0.5)),
    };

    let mut ground_geometry = GeometryInstance::new(ground_sphere_template.clone());
    ground_geometry
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            0.0, -1000.0, 0.0,
        )));
    let world = object::RenderObject {
        geometry_instance: ground_geometry,
        material_instance: MaterialInstance::new(Arc::new(lambertian::Lambertian::new(Box::new(
            checker::CheckerTexture::new(
                color::ColorTexture::new(vec::Vec3::new(0.2, 0.3, 0.1)),
                color::ColorTexture::new(vec::Vec3::new(0.9, 0.9, 0.9)),
                1.0,
            ),
        )))),
    };

    let skybox_primitive = Arc::new(world::World::new(
        &vec::Vec3::new(0.5, 0.7, 1.0),
        &vec::Vec3::new(1.0, 1.0, 1.0),
    ));
    let skybox = object::RenderObject {
        geometry_instance: GeometryInstance::new(skybox_primitive.clone()),
        material_instance: MaterialInstance::new(skybox_primitive.clone()),
    };

    scene.add_object(Box::new(center_sphere));
    scene.add_object(Box::new(left_sphere));
    scene.add_object(Box::new(right_sphere));
    scene.add_object(Box::new(world));
    scene.add_object(Box::new(skybox));
    scene.build_bvh(&mut rng);

    let render = render::Render {
        width: nx,
        samples: ns,
        depth: max_depth,
        camera,
        scene,
    };

    match scene_file::save_render(&render, &Path::new("scenes/bouncing_spheres.toml")) {
        Ok(_) => println!("Scene saved to scenes/bouncing_spheres.toml"),
        Err(e) => eprintln!("Failed to write scene file: {}", e),
    };

    if std::env::var("SKIP_RENDER").is_ok() {
        println!("SKIP_RENDER set; skipping image generation.");
        return;
    }

    println!(
        "Rendering a {}x{} image with {} samples per pixel and max depth {}",
        render.width,
        render.width as f32 * render.camera.aspect_ratio,
        render.samples,
        render.depth
    );

    let data = raytrace(&mut rng, &render);

    match image::save_buffer(
        &Path::new("samples/bouncing_spheres.png"),
        data.as_slice(),
        nx,
        ny,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to output.png"),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
