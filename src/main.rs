use std::{fs::File, io::Write};

use vec3::Vec3;

use crate::ray::Ray;

pub mod ray;
pub mod vec3;

fn main() {
    // let aspect_ratio = 16.0 / 9.0;
    let image_width = 200;
    // let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = 100;

    // let viewport_height = 2.0;
    // let viewport_width = aspect_ratio * viewport_height;
    // let focal_length = 1.0;

    // let coords_origin = vec3!(0, 0, 0);
    // let horizontal = vec3!(viewport_width, 0, 0);
    // let vertical = vec3!(0, viewport_height, 0);
    // let lower_left_corner =
    //     coords_origin.clone() - &horizontal / 2.0 - &vertical / 2.0 - vec3!(0, 0, focal_length);

    // !
    let lower_left_corner = vec3!(-2, -1, -1);
    let horizontal = vec3!(4, 0, 0);
    let vertical = vec3!(0, 2, 0);
    let coords_origin = vec3!(0, 0, 0);
    // !

    // write file

    let mut file = File::create("out.ppm").expect("Error encountered while creating file!");

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);

    file.write_all(header.as_bytes()).unwrap();

    for height_counter in (0..image_height).rev() {
        println!("Lines remaining: {}", height_counter);

        for width_counter in 0..image_width {
            let u = width_counter as f64 / image_width as f64;
            let v = height_counter as f64 / image_height as f64;

            let ray = Ray {
                origin: coords_origin.clone(),
                direction: lower_left_corner.clone() + &horizontal * u + &vertical * v,
            };

            let color = ray.get_color();
            let color_string = format!(
                "{}, {}, {}\t",
                (color.r() * 255.999) as i32,
                (color.g() * 255.999) as i32,
                (color.b() * 255.999) as i32
            );

            file.write_all(color_string.as_bytes())
                .expect("Can't write row info");
        }

        writeln!(&mut file, "\n").unwrap()
    }
}
