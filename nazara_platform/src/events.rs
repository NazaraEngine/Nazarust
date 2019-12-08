use winit::event_loop::EventLoop;

enum State {
	Pressed,
	Release,
}
enum KeyCode {
	Escape(State),
}

enum MouseEvent {
	Moved((u32,u32))
}