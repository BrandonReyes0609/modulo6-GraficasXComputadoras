use crate::fragment::Fragment;  // Cambiar a una importación directa
use crate::vertex::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::{Vec3, Vec2}; // Importar Vec2 aquí
use crate::utils::{calculate_bounding_box, barycentric_coordinates};

// Función para rasterizar un triángulo
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    // Dirección de la fuente de luz estática
    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    // Itera sobre cada píxel en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32, y as f32, 0.0);

            // Calcula las coordenadas baricéntricas
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c);

            // Verifica si el punto está dentro del triángulo
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                // Interpolación de normales
                let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
                let normal = normal.normalize();

                // Calcular la intensidad de la luz usando el producto punto
                let intensity = normal.dot(&light_dir).max(0.0);

                // Color base con iluminación
                let base_color = Color::new(100.0, 100.0, 100.0); // Gris medio

                // Interpolación de profundidad (z-index)
                let depth = w1 * a.z + w2 * b.z + w3 * c.z;

                // Crear el fragmento y agregarlo a la lista de fragmentos
                fragments.push(Fragment::new(
                    Vec2::new(x as f32, y as f32),
                    base_color,
                    depth,
                    normal,
                    intensity,
                ));
            }
        }
    }

    fragments
}
