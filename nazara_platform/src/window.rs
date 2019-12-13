use crate::winit_utility::{from_winit_event, NazarustEvents};
use std::{collections::HashMap, hash::Hash};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

use crate::events::{KeyEvent, MouseEvent, WindowEvent as NazarustWindowEvent};
pub struct Window {
    window: Option<WinitWindow>,
    window_builder: WinitWindowBuilder,
    name: String,
    resizable: bool,
    callbacks: HashMap<SimpleNazarustEvents, Box<dyn FnMut()>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SimpleNazarustEvents {
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent),
    WindowEvent(NazarustWindowEvent),
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
        Self {
            window: None,
            window_builder,
            name,
            resizable,
            callbacks,
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
                NazarustEvents::KeyEvent(event, _) => {
                    if let Some(lambda) = &mut self
                        .callbacks
                        .get_mut(&SimpleNazarustEvents::KeyEvent(event))
                    {
                        lambda();
                    }
                }
                NazarustEvents::MouseEvent(event, _) => {
                    if let Some(lambda) = &mut self
                        .callbacks
                        .get_mut(&SimpleNazarustEvents::MouseEvent(event))
                    {
                        lambda();
                    }
                }
                NazarustEvents::WindowEvent(event) => {
                    if let NazarustWindowEvent::CloseRequested = event {
                        if let None = &mut self
                            .callbacks
                            .get_mut(&SimpleNazarustEvents::WindowEvent(event))
                        {
                            *control_flow = ControlFlow::Exit;
                        }
                    } else {
                        if let Some(lambda) = &mut self
                            .callbacks
                            .get_mut(&SimpleNazarustEvents::WindowEvent(event))
                        {
                            lambda();
                        }
                    }
                }
                NazarustEvents::Unknown => (),
            };
        })
    }
    pub fn on_key_event(&mut self, event: KeyEvent, lambda: Box<dyn FnMut()>) {
        self.callbacks
            .insert(SimpleNazarustEvents::KeyEvent(event), lambda);
    }
    pub fn on_mouse_event(&mut self, event: MouseEvent, lambda: Box<dyn FnMut()>) {
        self.callbacks
            .insert(SimpleNazarustEvents::MouseEvent(event), lambda);
    }
    pub fn on_window_event(&mut self, event: NazarustWindowEvent, lambda: Box<dyn FnMut()>) {
        self.callbacks
            .insert(SimpleNazarustEvents::WindowEvent(event), lambda);
    }
}

#[derive(Clone)]
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
