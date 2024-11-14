use crate::vertex::vertex::Vertex;

pub struct Obj {
    pub vertices: Vec<Vertex>,
}

impl Obj {
    pub fn load(file_path: &str) -> Result<Self, String> {
        Ok(Self { vertices: Vec::new() }) // Reemplaza con la lógica para cargar vértices.
    }
}
