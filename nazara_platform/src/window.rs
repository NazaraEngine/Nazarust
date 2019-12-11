use winit::{
	dpi::LogicalSize,
	event::{Event as WinitEvent, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
	event_loop::{ControlFlow, EventLoop},
	window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

pub struct Window {
	window: Option<WinitWindow>,
	window_builder: WinitWindowBuilder,
	name: String,
	resizable: bool,
	callback: Box<dyn FnMut()>
}

impl Window {
	fn new(name: String, size: (u32, u32), resizable: bool) -> Self {
		let window_builder = WinitWindowBuilder::new()
			.with_title(&name)
			.with_inner_size(LogicalSize {
				height: size.0 as f64,
				width: size.1 as f64,
			});
		let callback = Box::new(move ||());
		Self {
			window: None,
			window_builder,
			name,
			resizable,
			callback
		}
	}
	pub fn run_loop(&mut self) {
		let event_loop = EventLoop::new();
		self.window = Some(self.window_builder.build(&event_loop).unwrap());
		event_loop.run(move |event, _, control_flow| {
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
										 (Some(VirtualKeyCode::A), ElementState::Pressed) => {
											(*self.callback)();
										},
										 _ => {},
									}
								},
							}
						},
						 _ => {},
					}
				}
			_ => (),
			}

		})
	}
	pub fn change_lambda<T: 'static + FnMut()>(&mut self, lambda: T) {
        self.callback = Box::new(lambda);
    }
}

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
