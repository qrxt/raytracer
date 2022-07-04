use crate::vec3;
use crate::vec3::Vec3;

type Color = Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    // pub fn at(self, t: f64) -> Vec3 {
    //     self.origin + &self.direction * t
    // }

    pub fn get_color(&self) -> Vec3 {
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);

        let white: Color = vec3!(1, 1, 1);
        let blue: Color = vec3!(0.5, 0.7, 1.0);

        &white * (1.0 - t) + &blue * t
    }
}

#[test]
fn test_get_color() {
    let ray1 = Ray {
        origin: vec3!(0, 0, 0),
        direction: vec3!(1, 2, 2),
    };

    let ray2 = Ray {
        origin: vec3!(0, 0, 0),
        direction: vec3!(-2, -2, -2),
    };

    let color_string1 = format!(
        "{}, {}, {}\t",
        (ray1.get_color().r() * 255.999) as i32,
        (ray1.get_color().g() * 255.999) as i32,
        (ray1.get_color().b() * 255.999) as i32
    );

    let color_string2 = format!(
        "{}, {}, {}\t",
        (ray2.get_color().r() * 255.999) as i32,
        (ray2.get_color().g() * 255.999) as i32,
        (ray2.get_color().b() * 255.999) as i32
    );

    println!("{:?} || {:?}", color_string1, color_string2);
}
