// Archivo: main.rs

mod uniforms;
mod vertex;
mod obj;
mod color;
mod vertex_shader;
mod camera;
mod framebuffer;
mod triangle;
mod fragment;
mod utils;
mod fragment_shader; // Importa el nuevo fragment shader

use crate::uniforms::{create_model_matrix, create_view_matrix, create_perspective_matrix, create_viewport_matrix, Uniforms};
use crate::obj::Obj;
use crate::color::Color;
use crate::vertex_shader::vertex_shader;
use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::fragment_shader::fragment_shader; // Importar el fragment shader
use nalgebra_glm::Vec3;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let width = 800;
    let height = 600;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Renderer Example")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(width as u32, height as u32, &window);
    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture).unwrap(); // Cambié a `mut pixels`

    // Crear la cámara
    let mut camera = Camera::new(Vec3::new(2.0, 1.0, -5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    // Crear las matrices de modelo, vista, proyección y viewport
    let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.0, Vec3::new(0.0, 0.0, 0.0));
    let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
    let projection_matrix = create_perspective_matrix(width as f32, height as f32);
    let viewport_matrix = create_viewport_matrix(width as f32, height as f32);

    let uniforms = Uniforms {
        model_matrix,
        view_matrix,
        projection_matrix,
        viewport_matrix,
    };

    // Precalcular la matriz de transformación completa fuera del vertex shader
    let transform_matrix = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix;

    // Cargar el modelo y aplicar las transformaciones
    let mut model = Obj::load("assets/squirtle.obj").expect("Failed to load OBJ file");
    for vertex in &mut model.vertices {
        *vertex = vertex_shader(vertex, &transform_matrix, &uniforms.viewport_matrix);
    }

    let mut framebuffer = Framebuffer::new(width, height);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::Left => camera.orbit(-0.05, 0.0),
                        VirtualKeyCode::Right => camera.orbit(0.05, 0.0),
                        VirtualKeyCode::Up => camera.orbit(0.0, -0.05),
                        VirtualKeyCode::Down => camera.orbit(0.0, 0.05),
                        VirtualKeyCode::W => camera.zoom(-0.1),
                        VirtualKeyCode::S => camera.zoom(0.1),
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                // Dibujar y renderizar
                framebuffer.clear(Color::black());

                // Rasterizar triángulos del modelo
                for triangle in model.triangles() {
                    let fragments = triangle::triangle(&triangle.0, &triangle.1, &triangle.2);
                    for fragment in fragments {
                        let x = fragment.position.x as usize;
                        let y = fragment.position.y as usize;
                        if x < framebuffer.width && y < framebuffer.height {
                            // Aplicar el fragment shader
                            let shaded_color = fragment_shader(&fragment, &uniforms);
                            framebuffer.set_current_color(shaded_color.to_hex());
                            framebuffer.point(x, y, fragment.depth);
                        }
                    }
                }

                // Copiar el framebuffer a la ventana de renderizado
                let frame = pixels.frame_mut(); // Cambié a `frame_mut()`
                frame.copy_from_slice(&framebuffer.to_vec());

                // Renderizar los píxeles
                if pixels.render().is_err() {
                    eprintln!("Error rendering frame");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
