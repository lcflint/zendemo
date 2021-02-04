use terrainbasic::{
    entities::*,
    input::*,
    models::raw_model::*,
    rendering::*,
    utils::*
};

use glutin::{
    event::{Event, KeyboardInput, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop}
};

use cgmath::{Vector3};

fn main() {
    // first, creates an event loop
    let event_loop = EventLoop::new();

    // creates the window
    let (context) = display_manager::create_display(&event_loop);

    // creates a new loader
    let mut loader = loader::Loader::default();

    // creates the renderer
    let mut renderer = renderer::Renderer::new();

    // creates a camera
    let mut camera = camera::Camera::new();

    // creates a new input manager
    let mut input_manager = input_manager::InputManager::new();

    // reads the triangulation table from the files and prepares conversion data
    let tri_table = table_reader::TriangulationTable::new();

    // creates a model vector
    let mut model_vector = Vec::<RawModel>::new();

    // for x in 0..8 {
    //     for y in 0..8 {
    //         for z in 0..8 {
    //             // creates a test chunk
    //             let chunk = basic_marching_cubes::MCChunk::new(
    //                 Vector3::new(x, y, z),
    //                 &tri_table
    //             );

    //             // creates a model
    //             let model = loader.load_to_vao(chunk.vertices, chunk.indices);

    //             // adds the model to the model vector
    //             model_vector.push(model);
    //         }
    //     }
    // }

    // creates a chunk manager
    let mut chunk_manager = chunk_manager::ChunkManager::new();

    // creates an fps counter
    let mut tick_counter = fps::FPSLimiter::new();
    
    // runs the event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    // resizes the viewport
                    context.resize(physical_size);
                    unsafe {
                        gl::Viewport(
                            0, 0, 
                            context.window().inner_size().width as i32, 
                            context.window().inner_size().height as i32
                        );
                    }
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    renderer.clean_up();
                },
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => input_manager.register_key_press(virtual_code),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Released,
                            ..
                        },
                    ..
                } => input_manager.register_key_release(virtual_code),
                _ => (),
            },
            | Event::MainEventsCleared => {
                context.window().request_redraw();
            },
            Event::RedrawRequested(_) => {

                // gets the current delta time
                let delta_time = tick_counter.delta_time();

                // gets the fps count and init the number of polys in the models
                let fps_count = tick_counter.fps().floor();
                let mut poly_count: u32 = 0;

                // loops over the models to add to the poly count
                for model in &mut model_vector {
                    let model_poly_count = model.get_vertex_count()/3;
                    poly_count = poly_count + (model_poly_count as u32);
                }

                // sets the title string
                let title_string = format!(
                    "Terrain Test | FPS: {} | Polygons: {}",
                    fps_count,
                    poly_count
                );

                // updates the title of the screen
                context.window().set_title(&title_string);

                // runs the camera
                camera.update(&mut input_manager, &delta_time);

                // generates chunks
                model_vector = chunk_manager.generate_chunk_models(&camera, &mut loader, &tri_table);

                // runs the renderer
                renderer.render(&mut model_vector, &mut camera);

                // swaps images on the swap chain
                context.swap_buffers().unwrap();

                // ticks the frames
                tick_counter.tick_frame();
            },
            _ => (),
        }
    });
}