use std::{f32::consts::PI, path::Path, sync::Arc};

use rustray::core::{camera, object, render, scene, scene_file};
use rustray::geometry::{
    instance::GeometryInstance,
    primitives::{cube, quad},
    transform,
};
use rustray::materials::{diffuse_light, instance::MaterialInstance, lambertian};
use rustray::math::{mat, vec};
use rustray::textures::color;

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

    let nx = 600;
    let ar = 1.0;
    let ny = (nx as f32 / ar) as u32;
    let ns = 1000;
    let max_depth = 100;

    let camera_config = camera::CameraConfig {
        origin: vec::Vec3::new(278.0, 278.0, -800.0),
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

    let red = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(0.65, 0.05, 0.05)),
    )));
    let green = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(0.12, 0.45, 0.15)),
    )));
    let white = Arc::new(lambertian::Lambertian::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(0.73, 0.73, 0.73)),
    )));
    let light = Arc::new(diffuse_light::DiffuseLight::new(Box::new(
        color::ColorTexture::new(vec::Vec3::new(15.0, 15.0, 15.0)),
    )));

    let left_wall = quad::Quad::new(
        vec::Vec3::new(0.0, 0.0, 555.0),
        vec::Vec3::new(0.0, 0.0, -555.0),
        vec::Vec3::new(0.0, 555.0, 0.0),
    );
    let right_wall = quad::Quad::new(
        vec::Vec3::new(555.0, 0.0, 0.0),
        vec::Vec3::new(0.0, 0.0, 555.0),
        vec::Vec3::new(0.0, 555.0, 0.0),
    );
    let floor = quad::Quad::new(
        vec::Vec3::new(0.0, 0.0, 0.0),
        vec::Vec3::new(0.0, 0.0, 555.0),
        vec::Vec3::new(555.0, 0.0, 0.0),
    );
    let ceiling = quad::Quad::new(
        vec::Vec3::new(0.0, 555.0, 555.0),
        vec::Vec3::new(0.0, 0.0, -555.0),
        vec::Vec3::new(555.0, 0.0, 0.0),
    );
    let back_wall = quad::Quad::new(
        vec::Vec3::new(555.0, 0.0, 555.0),
        vec::Vec3::new(-555.0, 0.0, 0.0),
        vec::Vec3::new(0.0, 555.0, 0.0),
    );
    let ceiling_light = quad::Quad::new(
        vec::Vec3::new(213.0, 554.0, 227.0),
        vec::Vec3::new(130.0, 0.0, 0.0),
        vec::Vec3::new(0.0, 0.0, 105.0),
    );

    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(left_wall)),
        material_instance: MaterialInstance::new(green.clone()),
    }));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(right_wall)),
        material_instance: MaterialInstance::new(red.clone()),
    }));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(floor)),
        material_instance: MaterialInstance::new(white.clone()),
    }));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(ceiling)),
        material_instance: MaterialInstance::new(white.clone()),
    }));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(back_wall)),
        material_instance: MaterialInstance::new(white.clone()),
    }));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: GeometryInstance::new(Arc::new(ceiling_light)),
        material_instance: MaterialInstance::new(light.clone()),
    }));

    let short_box_geom = Arc::new(cube::Cube::new(
        vec::Vec3::new(0.0, 0.0, 0.0),
        vec::Vec3::new(165.0, 165.0, 165.0),
    ));
    let tall_box_geom = Arc::new(cube::Cube::new(
        vec::Vec3::new(0.0, 0.0, 0.0),
        vec::Vec3::new(165.0, 330.0, 165.0),
    ));

    let mut short_box_instance = GeometryInstance::new(short_box_geom.clone());
    short_box_instance
        .transforms
        .push(transform::Transform::Rotate(rotation_y(-18.0)));
    short_box_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            130.0, 0.0, 65.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: short_box_instance,
        material_instance: MaterialInstance::new(white.clone()),
    }));

    let mut tall_box_instance = GeometryInstance::new(tall_box_geom.clone());
    tall_box_instance
        .transforms
        .push(transform::Transform::Rotate(rotation_y(15.0)));
    tall_box_instance
        .transforms
        .push(transform::Transform::Translate(vec::Vec3::new(
            265.0, 0.0, 295.0,
        )));
    scene.add_object(Box::new(object::RenderObject {
        geometry_instance: tall_box_instance,
        material_instance: MaterialInstance::new(white.clone()),
    }));

    scene.build_bvh(&mut rng);

    let render = render::Render {
        width: nx,
        samples: ns,
        depth: max_depth,
        camera,
        scene,
    };

    match scene_file::save_render(&render, &Path::new("scenes/cornell_box.toml")) {
        Ok(_) => println!("Scene saved to scenes/cornell_box.toml"),
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
        &Path::new("samples/cornell_box.png"),
        data.as_slice(),
        nx,
        ny,
        image::ColorType::Rgb8,
    ) {
        Ok(_) => println!("Image saved to samples/cornell_box.png"),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
