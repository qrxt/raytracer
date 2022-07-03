use vec3::Vec3;

mod color;
mod vec3;

fn main() {
    let vec1 = Vec3::new((1.0, 3.0, 0.0));
    let vec2 = Vec3::new((2.0, 2.0, 0.0));

    println!("{:?}", vec1 + vec2);
}
