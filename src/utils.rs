// Archivo: utils.rs
pub use self::utils::calculate_bounding_box;
pub use self::utils::barycentric_coordinates;

pub mod utils {
    use nalgebra_glm::Vec3;

    pub fn calculate_bounding_box(a: &Vec3, b: &Vec3, c: &Vec3) -> (i32, i32, i32, i32) {
        let min_x = a.x.min(b.x).min(c.x).floor() as i32;
        let min_y = a.y.min(b.y).min(c.y).floor() as i32;
        let max_x = a.x.max(b.x).max(c.x).ceil() as i32;
        let max_y = a.y.max(b.y).max(c.y).ceil() as i32;

        (min_x, min_y, max_x, max_y)
    }

    pub fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);

        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        (u, v, w)
    }
}
