pub mod vertex {
    use nalgebra_glm::Vec3;

    #[derive(Clone, Copy, Debug)]
    pub struct Vertex {
        pub position: Vec3,
        pub transformed_position: Vec3,
        pub transformed_normal: Vec3,
        pub color: Vec3,
        pub normal: Vec3,           // Añadir el campo `normal`.
        pub tex_coords: Vec3,       // Añadir el campo `tex_coords`.
    }
}
