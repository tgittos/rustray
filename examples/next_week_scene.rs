use rand::Rng;
use std::{f32::consts::PI, path::Path, sync::Arc};

use rustray::core::{camera, object, render, scene, scene_file, volume};
use rustray::geometry::{
    instance::GeometryInstance,
    primitives::{cube, quad, sphere},
    transform,
};
use rustray::materials::{
    dielectric, diffuse_light, instance::MaterialInstance, lambertian, metallic,
};
use rustray::math::{mat, vec};
use rustray::textures::{color, noise, uv};

use rustray::raytrace;

fn rotation_y(angle_degrees: f32) -> mat::Mat3 {
    let theta = angle_degrees * (PI / 180.0);
    let (sin_t, cos_t) = theta.sin_cos();
    mat::Mat3::new([
        vec::Vec3::new(cos_t, 0.0, sin_t),
        vec::Vec3::new(0.0, 1.0, 0.0),
        vec::Vec3::new(-sin_t, 0.0, cos_t),
    ])
}

fn main() {
    let mut rng = rand::rng();

    let nx = 800;
    let ar = 1.0;
    let ny = (nx as f32 / ar) as u32;
    let ns = 10000;
    let max_depth = 40;

    let camera_config = camera::CameraConfig {
        origin: vec::Vec3::new(478.0, 278.0, -600.0),
        look_at: vec::Vec3::new(278.0, 278.0, 0.0),
        up: vec::Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio: ar,
        viewport_height: 2.0,
        focal_length: 1.0,
        aperture: 0.0,
        vertical_fov: 40.0,
    };
    let camera = camera::Camera::with_config(camera_config);
    let mut scene = scene::Scene::new();

    let ground_mat = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(0.48, 0.83, 0.53)),
    )));
    let white_mat = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(0.73, 0.73, 0.73)),
    )));
    let light_mat = Arc::new(diffuse_light::DiffuseLight::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(7.0, 7.0, 7.0)),
    )));
    let center_mat = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(0.7, 0.3, 0.1)),
    )));
    let glass_mat = Arc::new(dielectric::Dielectric::new(1.5));
    let metal_mat = Arc::new(metallic::Metallic::new(&vec::Vec3::new(0.8, 0.8, 0.9), 1.0));
    let earth_mat = Arc::new(lambertian::Lambertian::new(Box::new(uv::UvTexture::new(
        "assets/earth.jpg",
    ))));
    let perlin_mat = Arc::new(lambertian::Lambertian::new(Box::new(
        noise::NoiseTexture::new(&mut rng, 0.2),
    )));

    // Ground boxes grid
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y1: f32 = rng.random_range(1.0..101.0);
            let x1 = x0 + w;
            let z1 = z0 + w;

            let box_geom = cube::Cube::new(vec::Vec3::new(x0, 0.0, z0), vec::Vec3::new(x1, y1, z1));
            scene.add_object(Box::new(object::RenderObject {
                geometry_instance: GeometryInstance::new(Arc::new(box_geom)),
                material_instance: MaterialInstance::new(ground_mat.clone()),
            }));
        }
    }

    // Ceiling light
    let light_quad = quad::Quad::new(
        vec::Vec3::new(123.0, 554.0, 147.0),
        vec::Vec3::new(300.0, 0.0, 0.0),
        vec::Vec3::new(0.0, 0.0, 265.0),
    );
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(light_quad)),
        material_instance: MaterialInstance::new(light_mat.clone()),
    }));

    // Moving sphere
    let moving_sphere_geom = Arc::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, 0.0), 50.0));
    let mut moving_instance = GeometryInstance::new(moving_sphere_geom.clone());
    moving_instance.transforms.push(transform::Transform::Move {
        start: vec::Vec3::new(0.0, 0.0, 0.0),
        end: vec::Vec3::new(30.0, 0.0, 0.0),
        time_start: 0.0,
        time_end: 1.0,
    });
    moving_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            400.0, 400.0, 200.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: moving_instance,
        material_instance: MaterialInstance::new(center_mat.clone()),
    }));

    // Static glass and metal spheres
    let mut glass_instance = GeometryInstance::new(Arc::new(sphere::Sphere::new(
        &vec::Vec3::new(0.0, 0.0, 0.0),
        50.0,
    )));
    glass_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            260.0, 150.0, 45.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: glass_instance,
        material_instance: MaterialInstance::new(glass_mat.clone()),
    }));

    let mut metal_instance = GeometryInstance::new(Arc::new(sphere::Sphere::new(
        &vec::Vec3::new(0.0, 0.0, 0.0),
        50.0,
    )));
    metal_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            0.0, 150.0, 145.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: metal_instance,
        material_instance: MaterialInstance::new(metal_mat.clone()),
    }));

    // Boundary glass sphere and blue volume
    let boundary_geom = Arc::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, 0.0), 70.0));
    let mut boundary_instance = GeometryInstance::new(boundary_geom.clone());
    boundary_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            360.0, 150.0, 145.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: boundary_instance,
        material_instance: MaterialInstance::new(glass_mat.clone()),
    }));

    let mut volume_boundary = GeometryInstance::new(boundary_geom.clone());
    volume_boundary
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            360.0, 150.0, 145.0,
        )));
    scene.add_object(Box::new(volume::RenderVolume::new(
        Box::new(volume_boundary),
        0.2,
        Arc::new(volume::Isotropic::new(Box::new(color::ColorTexture::new(
            vec::Vec3::new(0.2, 0.4, 0.9),
        )))),
    )));

    // Giant white fog volume
    let world_boundary = GeometryInstance::new(Arc::new(sphere::Sphere::new(
        &vec::Vec3::new(0.0, 0.0, 0.0),
        5000.0,
    )));
    scene.add_object(Box::new(volume::RenderVolume::new(
        Box::new(world_boundary),
        0.0001,
        Arc::new(volume::Isotropic::new(Box::new(color::ColorTexture::new(
            vec::Vec3::new(1.0, 1.0, 1.0),
        )))),
    )));

    // Earth and Perlin spheres
    let mut earth_instance = GeometryInstance::new(Arc::new(sphere::Sphere::new(
        &vec::Vec3::new(0.0, 0.0, 0.0),
        100.0,
    )));
    earth_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            400.0, 200.0, 400.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: earth_instance,
        material_instance: MaterialInstance::new(earth_mat.clone()),
    }));

    let mut perlin_instance = GeometryInstance::new(Arc::new(sphere::Sphere::new(
        &vec::Vec3::new(0.0, 0.0, 0.0),
        80.0,
    )));
    perlin_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            220.0, 280.0, 300.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: perlin_instance,
        material_instance: MaterialInstance::new(perlin_mat.clone()),
    }));

    // Cluster of small spheres
    let small_sphere_geom = Arc::new(sphere::Sphere::new(&vec::Vec3::new(0.0, 0.0, 0.0), 10.0));
    let cluster_rotation = rotation_y(15.0);
    for _ in 0..1000 {
        let center = vec::Vec3::new(
            rng.random_range(0.0..165.0),
            rng.random_range(0.0..165.0),
            rng.random_range(0.0..165.0),
        );
        let mut instance = GeometryInstance::new(small_sphere_geom.clone());
        instance
            .transforms
            .push(transform::Transform::Translate(center));
        instance
            .transforms
            .push(transform::Transform::Rotate(cluster_rotation));
        instance
            .transforms
            .push(transform::Transform::Translate(vec::Vec3::new(
                -100.0, 270.0, 395.0,
            )));

        scene.add_object(Box::new(object::RenderObject {
            geometry_instance: instance,
            material_instance: MaterialInstance::new(white_mat.clone()),
        }));
    }

    scene.build_bvh(&mut rng);

    let render = render::Render {
        width: nx,
        samples: ns,
        depth: max_depth,
        camera,
        scene,
    };

    match scene_file::save_render(&render, &Path::new("scenes/next_week_scene.toml")) {
        Ok(_) => println!("Scene saved to scenes/next_week_scene.toml"),
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
        &Path::new("samples/next_week_scene.png"),
        data.as_slice(),
        nx,
        ny,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to output.png"),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
