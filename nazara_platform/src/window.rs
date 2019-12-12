use crate::winit_utility::{NazarustEvent, from_winit_event};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::hash::Hash;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event as WinitEvent, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

use crate::events::{KeyEvent, MouseEvent, WindowEvent as NazarustWindowEvent};

pub struct Window<T: NazarustEvent> {
    window: Option<WinitWindow>,
    window_builder: WinitWindowBuilder,
    name: String,
    resizable: bool,
    callbacks: HashMap<SimpleNazarustEvents<T>, Box<dyn FnMut()>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SimpleNazarustEvents<T: NazarustEvent>{
	KeyEvent(T),
	WindowEvent(T),
	MouseEvent(T)
}
impl<T: NazarustEvent> Window<T> {
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
    pub fn run_loop(&mut self) {
        let event_loop = EventLoop::new();
        self.window = Some(self.window_builder.clone().build(&event_loop).unwrap());
        //let callback = Rc::clone(&self.callback);
        event_loop.run(move |event, _, control_flow| {
        	let mut nazarust_event = from_winit_event(event);
        	//nazarust_event = 
            //(&mut *self.callbacks[nazarust_event].borrow_mut())();
        })
    }
    pub fn set_callback<K: NazarustEvent>(&mut self, event: K, lambda: Box<dyn FnMut()>) {
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
    pub fn build_with<T: NazarustEvent>(&self, name: &'b str, size: (u32, u32)) -> Window<T> {
        Window::new(name.to_string(), size, self.resizable)
    }
}



