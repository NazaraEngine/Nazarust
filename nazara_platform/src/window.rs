use super::window_application::{WindowApplication, WindowCallbacks};
use std::{cell::RefCell, rc::Rc};
use winit::window::{Window as W_Window, WindowBuilder as W_WindowBuilder};

pub struct Window {
    handle: W_Window,
    callbacks: Rc<RefCell<WindowCallbacks>>,
}

impl Window {
    pub fn new(app: &mut WindowApplication) -> Window {
        let handle = W_Window::new(app.get_event_loop()).unwrap();

        Self::new_from_handle(app, handle)
    }

    pub fn set_render_callback<F>(&mut self, func: F)
    where
        F: FnMut() -> () + 'static,
    {
        self.callbacks.borrow_mut().render_callback = Box::new(func);
    }

    pub(crate) fn new_from_handle(app: &mut WindowApplication, handle: W_Window) -> Window {
        let callbacks = app.register_window(handle.id());

        Window { handle, callbacks }
    }
}

pub struct WindowBuilder {
    title: String,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            title: String::from("untitled Nazara window"),
        }
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn build(self, app: &mut WindowApplication) -> Window {
        let wbuilder = W_WindowBuilder::new();
        let window = wbuilder.with_title(self.title).build(app.get_event_loop());

        Window::new_from_handle(app, window.unwrap())
    }
}
