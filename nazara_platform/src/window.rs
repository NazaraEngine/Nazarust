use crate::winit_utility::{from_winit_event, NazarustEvents};
use std::{collections::HashMap, hash::Hash};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

use crate::events::{KeyEvent, MouseEvent, WindowEvent as NazarustWindowEvent};

#[allow(dead_code)]
pub struct Window {
    window: Option<WinitWindow>,
    window_builder: WinitWindowBuilder,
    name: String,
    resizable: bool,
    callbacks: HashMap<SimpleNazarustEvents, Box<dyn FnMut()>>,
    mouse_moved_callback: Box<dyn FnMut((f64, f64))>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SimpleNazarustEvents {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Window(NazarustWindowEvent),
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
        let callbacks = HashMap::new();
        let mouse_moved_callback = Box::new(|_: (f64, f64)| {});
        Self {
            window: None,
            window_builder,
            name,
            resizable,
            callbacks,
            mouse_moved_callback,
        }
    }
    pub fn run_loop(mut self) {
        let event_loop = EventLoop::new();
        self.window = Some(self.window_builder.clone().build(&event_loop).expect(
            "Window 
creation failed",
        ));
        event_loop.run(move |event, _, control_flow| {
            let nazarust_event = from_winit_event(event);
            match nazarust_event {
                NazarustEvents::KeyEvent(event) => {
                    if let Some(lambda) =
                        &mut self.callbacks.get_mut(&SimpleNazarustEvents::Key(event))
                    {
                        lambda();
                    }
                }
                NazarustEvents::MouseEvent(event, position) => {
                    if let MouseEvent::Moved = event {
                        let position = position.unwrap();
                        (self.mouse_moved_callback)((position.x, position.y));
                    } else if let Some(lambda) =
                        self.callbacks.get_mut(&SimpleNazarustEvents::Mouse(event))
                    {
                        lambda();
                    }
                }
                NazarustEvents::WindowEvent(event) => {
                    if let NazarustWindowEvent::CloseRequested = event {
                        if self
                            .callbacks
                            .get_mut(&SimpleNazarustEvents::Window(event))
                            .is_none()
                        {
                            *control_flow = ControlFlow::Exit;
                        }
                    } else if let Some(lambda) =
                        &mut self.callbacks.get_mut(&SimpleNazarustEvents::Window(event))
                    {
                        lambda();
                    }
                }
                NazarustEvents::Unknown => (),
            };
        })
    }
    pub fn on_key_event(&mut self, event: KeyEvent, lambda: Box<dyn FnMut()>) {
        self.callbacks
            .insert(SimpleNazarustEvents::Key(event), lambda);
    }
    pub fn on_mouse_event(&mut self, event: MouseEvent, lambda: Box<dyn FnMut()>) {
        if let MouseEvent::Moved = event {
            panic!("Use `on_mouse_moved_event` function instead of `on_mouse_event` for MouseEvent::Moved
callback");
        }
        self.callbacks
            .insert(SimpleNazarustEvents::Mouse(event), lambda);
    }
    pub fn on_window_event(&mut self, event: NazarustWindowEvent, lambda: Box<dyn FnMut()>) {
        self.callbacks
            .insert(SimpleNazarustEvents::Window(event), lambda);
    }
    pub fn on_mouse_moved_event(&mut self, lambda: Box<dyn FnMut((f64, f64))>) {
        self.mouse_moved_callback = lambda;
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
