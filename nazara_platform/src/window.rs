use crate::{
    events::{KeyEvent, MouseEvent, WindowEvent as NazarustWindowEvent},
    winit_utility::{from_winit_event, NazarustEvent},
};
pub use winit::event_loop::ControlFlow;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

#[derive(Clone, Debug, PartialEq)]
pub enum NazarustEvents {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Window(NazarustWindowEvent),
}

#[allow(dead_code)]
pub struct Window {
    window: Option<WinitWindow>,
    window_builder: WinitWindowBuilder,
    name: String,
    resizable: bool,
}
impl Window {
    fn new(name: String, size: (u32, u32), resizable: bool) -> Self {
        let window_builder =
            WinitWindowBuilder::new()
                .with_title(&name)
                .with_inner_size(LogicalSize {
                    height: size.0 as f64,
                    width: size.1 as f64,
                });
        Self {
            window: None,
            window_builder,
            name,
            resizable,
        }
    }
    pub fn run_loop(
        mut self,
        mut callback: Box<dyn FnMut(NazarustEvents, &mut ControlFlow) + 'static>,
    ) {
        let event_loop = EventLoop::new();
        self.window = Some(
            self.window_builder
                .clone()
                .build(&event_loop)
                .expect("Window creation failed"),
        );
        event_loop.run(move |event, _, control_flow| {
            let nazarust_event = from_winit_event(event);
            match nazarust_event {
                NazarustEvent::KeyEvent(event) => {
                    callback(NazarustEvents::Key(event), control_flow);
                }
                NazarustEvent::MouseEvent(event) => {
                    callback(NazarustEvents::Mouse(event), control_flow);
                }
                NazarustEvent::WindowEvent(event) => {
                    callback(NazarustEvents::Window(event), control_flow);
                }
                NazarustEvent::Unknown => (),
            };
        })
    }
}

#[derive(Clone, Default)]
pub struct WindowBuilder<'b> {
    name: &'b str,
    size: (u32, u32),
    resizable: bool,
}

impl<'b> WindowBuilder<'b> {
    pub fn new() -> Self {
        Self {
            name: "Nazarust",
            size: (0, 0),
            resizable: false,
        }
    }
    pub fn with_resizable(mut self) -> WindowBuilder<'b> {
        self.resizable = true;
        self
    }
    pub fn build_with(&self, name: &'b str, size: (u32, u32)) -> Window {
        Window::new(name.to_string(), size, self.resizable)
    }
}
