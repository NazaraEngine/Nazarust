use winit::{
    dpi::LogicalSize,
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};

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