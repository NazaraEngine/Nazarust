use nazara_platform::{
    window::{Window, WindowBuilder},
    window_application::WindowApplication,
};

fn main() {
    let mut app = WindowApplication::new();

    let mut window = WindowBuilder::new()
        .with_title("Hello Nazara")
        .build(&mut app);

    window.set_render_callback(|| {
        println!("Render callback!");
    });

    app.run();
}
