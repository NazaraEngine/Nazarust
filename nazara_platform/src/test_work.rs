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
            state: State::Pressed,
            mouse_button: MouseButton::Right,
        },
        Box::new(move || {
            println!("{}", y);
        }),
    );
    window.run_loop();
}
