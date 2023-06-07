// TODO: Split into functions/refactor/learn more about whats going on.
// TODO: Can I add debug information to the window?

#[macro_use]
extern crate glium;

fn main() {
    use crate::glium::Surface;

    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_title("Hello World");
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [-0.5, 0.5] };
    let vertex3 = Vertex { position: [0.5, 0.5] };
    let vertex4 = Vertex { position: [0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    const TARGET_FPS: u64 = 10;

    let vertex_shader_src = r#"
     #version 140
     in vec2 position;
     uniform mat4 matrix;
     out vec2 my_attr;      

     void main() {
      my_attr = position;
      gl_Position = matrix * vec4(position, 0.0, 1.0);
     }
    "#;

    let fragment_shader_src = r#"
     #version 140

     in vec2 my_attr;
     out vec4 color;

     void main() {
      color = vec4(my_attr, 0.0, 1.0);  
     }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 0.0;
    let mut delta_per_second: f32 = 5.0;
    let mut delta_per_second_buffer: f32 = 0.0;

    event_loop.run(move | event, _, control_flow| {
        let start_time = std::time::Instant::now();

        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glium::glutin::event::WindowEvent::KeyboardInput { input, .. } =>
                    if input.state == glium::glutin::event::ElementState::Pressed {
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                glium::glutin::event::VirtualKeyCode::D => delta_per_second = -delta_per_second,
                                glium::glutin::event::VirtualKeyCode::E => delta_per_second = delta_per_second * 1.1,
                                glium::glutin::event::VirtualKeyCode::X => delta_per_second = delta_per_second * 0.9,
                                glium::glutin::event::VirtualKeyCode::S => delta_per_second = if delta_per_second == 0.0 {
                                    delta_per_second_buffer
                                } else {
                                    delta_per_second_buffer = delta_per_second;
                                    0.0
                                },
                                _ => {}
                            }
                        }
                    },
                _ => return,
            },
            glium::glutin::event::Event::NewEvents(cause) => match cause {
                glium::glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glium::glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0
        };
        let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(new_inst);

        t += delta_per_second / TARGET_FPS as f32;

       // if (t > std::f32::consts::PI) || (t< -std::f32::consts::PI) {
       //     delta_per_second = -delta_per_second
       // }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
