use nazara_platform::{
    events::{MouseEvent, WindowEvent},
    window::{ControlFlow, NazarustEvents, WindowBuilder},
};
fn main() {
    let window = WindowBuilder::new().build_with("My window", (400, 600));
    window.run_loop(Box::new(move |event, control_flow| match event {
        NazarustEvents::Mouse(mouse) => match mouse{
            MouseEvent::Moved(position) => println!("{:?}",position),
            _ => ()
        },
        NazarustEvents::Window(WindowEvent::CloseRequested) => *control_flow = ControlFlow::Exit,
        _ => (),
    }));
}