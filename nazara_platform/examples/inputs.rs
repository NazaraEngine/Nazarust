use nazara_platform::{
    events::{KeyEvent, MouseButton, MouseEvent, State, WindowEvent},
    window::WindowBuilder,
};
fn main() {
    let mut window = WindowBuilder::new().build_with("My window", (400, 600));
    let x = "Key A pressed";
    let y = "Right mouse button clicked";
    window.on_key_event(
        KeyEvent::A {
            state: State::Pressed,
        },
        Box::new(move || {
            println!("{}", x);
        }),
    );
    window.on_mouse_event(
        MouseEvent::Button {
            state: State::Released,
            mouse_button: MouseButton::Middle,
        },
        Box::new(move || {
            println!("Middle button released");
        }),
    );
    window.on_window_event(
        WindowEvent::Resized,
        Box::new(move || {
            println!("Resized !");
        }),
    );
    window.on_mouse_moved_event(Box::new(move |pos: (f64, f64)| {
        if pos.0 < 50.0 && pos.1 > 200.0 {
            println!("{:#?}", pos);
        }
    }));
    window.run_loop();
}
