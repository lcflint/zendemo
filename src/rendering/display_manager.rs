

use std::sync::mpsc::{Receiver};

use glutin::{
    ContextBuilder,
    PossiblyCurrent,
    WindowedContext,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

pub fn create_display(event_loop: &EventLoop<()>) -> WindowedContext<PossiblyCurrent> {

    // creates a new window using the windowbuilder
    let window = WindowBuilder::new()
        .with_title("Test");

    // creates a new context wrapper using the contextbuilder
    let context = ContextBuilder::new()
        .build_windowed(window, event_loop)
        .unwrap();

    // makes the window the current context
    let context = unsafe { context.make_current().unwrap() };

    // loads the opengl function pointers
    gl::load_with(|symbol| context.context().get_proc_address(symbol) as *const _);

    // sets the viewport size
    unsafe {
        gl::Viewport(
            0, 0, 
            context.window().inner_size().width as i32, 
            context.window().inner_size().height as i32
        );
    }

    context
}