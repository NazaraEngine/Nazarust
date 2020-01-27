/// Represents a Nazara application with Windows
use nazara_core::application::Application;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub(crate) struct WindowCallbacks {
    pub(crate) render_callback: Box<dyn FnMut() -> ()>,
}

pub struct WindowApplication {
    app: Application,
    event_loop: Option<EventLoop<()>>,
    window_callbacks: HashMap<WindowId, Rc<RefCell<WindowCallbacks>>>,
}

impl WindowApplication {
    pub fn new() -> WindowApplication {
        WindowApplication {
            app: Application::new(),
            event_loop: Some(EventLoop::new()),
            window_callbacks: HashMap::new(),
        }
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, window_target, control_flow| match event {
            Event::MainEventsCleared => {
                if !self.app.execute() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(win_id) => {
                if let Some(callback) = self.window_callbacks.get(&win_id) {
                    let callback = &mut callback.borrow_mut().render_callback;
                    callback();
                }
            }
            _ => (),
        });
    }

    pub(crate) fn get_event_loop(&self) -> &EventLoop<()> {
        assert!(self.event_loop.is_some());

        self.event_loop.as_ref().unwrap()
    }

    pub(crate) fn register_window(&mut self, id: WindowId) -> Rc<RefCell<WindowCallbacks>> {
        let callbacks = WindowCallbacks {
            render_callback: Box::new(|| {}),
        };

        let callback_ref = RefCell::new(callbacks);
        let callback_rc = Rc::new(callback_ref);
        self.window_callbacks.insert(id, callback_rc.clone());

        callback_rc
    }
}
