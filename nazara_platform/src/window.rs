use winit::{
    dpi::LogicalSize,
    event::{Event as WinitEvent, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

pub struct Window<'a> {
    window: WinitWindow,
    event_loop: EventLoop<()>,
    name: &'a str,
    resizable: bool,
}

impl<'a> Window<'a> {
    fn new(name: &'a str, size: (u32, u32), resizable: bool) -> Window<'a> {
        let event_loop = EventLoop::new();
        let window = WinitWindowBuilder::new()
            .with_title(name)
            .with_inner_size(LogicalSize {
                height: size.0 as f64,
                width: size.1 as f64,
            })
            .build(&event_loop)
            .unwrap();
        Window {
            window,
            event_loop,
            name,
            resizable,
        }
    }
    pub fn run_loop(mut self) {

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                 WinitEvent::WindowEvent { event, .. } => {
                    match event {
                         WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                         WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                 KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                         (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            *control_flow = ControlFlow::Exit
                                        },
                                         _ => {},
                                    }
                                },
                            }
                        },
                         _ => {},
                    }
                },
                _ => (),
            }

        })
    }
}

pub struct WindowBuilder<'a> {
    name: &'a str,
    size: (u32, u32),
    resizable: bool,
}

impl<'a> WindowBuilder<'a> {
    pub fn new() -> WindowBuilder<'a> {
        WindowBuilder {
            name: "Nazarust",
            size: (0, 0),
            resizable: false,
        }
    }
    pub fn with_resizable(mut self) -> WindowBuilder<'a> {
        self.resizable = true;
        self
    }
    pub fn build_with(&self, name: &'a str, size: (u32, u32)) -> Window<'_> {
            Window::new(name, size, self.resizable)
    }
}
