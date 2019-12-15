use crate::events::{
    KeyEvent, MouseButton as NazarustMouseButton, MouseEvent, State,
    WindowEvent as NazarustWindowEvent,
};
use winit::event::{
    ElementState, Event as WinitEvent, KeyboardInput, MouseButton as WinitMouseButton,
    VirtualKeyCode, WindowEvent,
};
pub enum NazarustEvent {
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent),
    WindowEvent(NazarustWindowEvent),
    Unknown,
}
pub fn from_winit_event<K>(winit_event: WinitEvent<K>) -> NazarustEvent {
    match winit_event {
        WinitEvent::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(_) => NazarustEvent::WindowEvent(NazarustWindowEvent::Resized),
            WindowEvent::Moved(_) => NazarustEvent::WindowEvent(NazarustWindowEvent::Moved),
            WindowEvent::CloseRequested => {
                NazarustEvent::WindowEvent(NazarustWindowEvent::CloseRequested)
            }
            WindowEvent::CursorMoved { position, .. } => {
                NazarustEvent::MouseEvent(MouseEvent::Moved(position))
            }
            WindowEvent::MouseInput { state, button, .. } => {
                ////let nazarust_modifiers = winit_to_nazarust_modifiers(modifiers);
                match (button, state) {
                    (WinitMouseButton::Left, ElementState::Pressed) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Left,
                            state: State::Pressed,
                        })
                    }
                    (WinitMouseButton::Left, ElementState::Released) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Left,
                            state: State::Released,
                        })
                    }
                    (WinitMouseButton::Right, ElementState::Pressed) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Right,
                            state: State::Pressed,
                        })
                    }
                    (WinitMouseButton::Right, ElementState::Released) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Right,
                            state: State::Released,
                        })
                    }
                    (WinitMouseButton::Middle, ElementState::Pressed) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Middle,
                            state: State::Pressed,
                        })
                    }
                    (WinitMouseButton::Middle, ElementState::Released) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Middle,
                            state: State::Released,
                        })
                    }
                    (WinitMouseButton::Other(n), ElementState::Pressed) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Other(n),
                            state: State::Pressed,
                        })
                    }
                    (WinitMouseButton::Other(n), ElementState::Released) => {
                        NazarustEvent::MouseEvent(MouseEvent::Button {
                            mouse_button: NazarustMouseButton::Other(n),
                            state: State::Released,
                        })
                    }
                }
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    virtual_keycode,
                    state,
                    ..
                } => {
                    //let nazarust_modifiers = winit_to_nazarust_modifiers(modifiers);
                    match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::A {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::B), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::B {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::C), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::C {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::D {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::E), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::E {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::G), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::G {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::H), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::H {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::I), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::I {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::J), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::J {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::K), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::K {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::L), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::L {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::M), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::M {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::N), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::N {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::O), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::O {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::P), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::P {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Q {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::R), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::R {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::S {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::T), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::T {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::U), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::U {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::V), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::V {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::W), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::W {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::X), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::X {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Y), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Y {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Z), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Z {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key1 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key2 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key3), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key3 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key4), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key4 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key5), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key5 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key6), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key6 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key7), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key7 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key8), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key8 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Key9), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key9 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Escape {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F1 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F2 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F3), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F3 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F4), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F4 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F5), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F5 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F6), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F6 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F7), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F7 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F8), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F8 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F9), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F9 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F10), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F10 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F11), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F11 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F12), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F12 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F13), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F13 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F14), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F14 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F15), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F15 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F16), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F16 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F17), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F17 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F18), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F18 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F19), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F19 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F20), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F20 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F21), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F21 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F22), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F22 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F23), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F23 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::F24), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::F24 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Snapshot), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Snapshot {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Scroll), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::ScrollLock {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Pause), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Pause {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Insert), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Insert {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Home), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Home {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Delete), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Delete {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::End), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::End {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::PageDown), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::PageDown {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::PageUp), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::PageUp {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Left), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Left {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Up {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Right), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Right {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Down), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Down {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Back), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Back {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Return), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Enter {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Space {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Compose), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Compose {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Caret), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Caret {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numlock), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numlock {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad0), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad0 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad1 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad2 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad3), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad3 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad4), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad4 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad5), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad5 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad6), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad6 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad7), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad7 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad8), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad8 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad9), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad9 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::AbntC1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::AbntC1 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::AbntC2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::AbntC2 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Add), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Add {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Apostrophe), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Apostrophe {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Apps), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Apps {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::At), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::At {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Ax), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Ax {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Backslash), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Backslash {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Calculator), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Calculator {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Capital), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Capital {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Colon), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Colon {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Comma), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Comma {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Convert), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Convert {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Decimal), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Decimal {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Divide), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Divide {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Equals), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Equals {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Grave), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Grave {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Kana), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Kana {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Kanji), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Kanji {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::LAlt), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::LBracket {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::LControl), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::LShift {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::LWin), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::LWin {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Mail), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Mail {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::MediaSelect), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::MediaSelect {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::MediaStop), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::MediaStop {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Minus), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Minus {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Multiply), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Multiply {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Mute), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Mute {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::MyComputer), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::MyComputer {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NavigateForward), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NavigateForward {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NavigateBackward), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NavigateBackward {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NextTrack), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NextTrack {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NoConvert), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NoConvert {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NumpadComma), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NumpadComma {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NumpadEnter), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NumpadEnter {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::NumpadEquals), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::NumpadEquals {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::OEM102), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::OEM102 {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Period), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Period {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::PlayPause), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::PlayPause {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Power), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Power {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::PrevTrack), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::PrevTrack {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::RAlt), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::RAlt {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::RBracket), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::RBracket {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::RControl), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::RControl {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::RShift), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::RShift {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::RWin), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::RWin {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Semicolon), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Semicolon {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Slash), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Slash {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Sleep), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Sleep {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Stop), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Stop {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Subtract), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Subtract {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Sysrq), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Sysrq {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Tab), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Tab {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Underline), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Underline {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Unlabeled), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Unlabeled {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::VolumeDown), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::VolumeDown {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::VolumeUp), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::VolumeUp {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Wake), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Wake {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebBack), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebBack {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebFavorites), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebFavorites {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebForward), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebForward {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebHome), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebHome {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebRefresh), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebRefresh {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebSearch), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebSearch {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::WebStop), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebStop {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Yen), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Yen {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Copy), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Copy {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Paste), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Paste {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::Cut), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyEvent::Cut {
                                state: State::Pressed,
                            })
                        }
                        (Some(VirtualKeyCode::A), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::A {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::B), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::B {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::C), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::C {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::D), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::D {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::E), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::E {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::G), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::G {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::H), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::H {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::I), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::I {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::J), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::J {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::K), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::K {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::L), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::L {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::M), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::M {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::N), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::N {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::O), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::O {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::P), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::P {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Q {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::R), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::R {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::S), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::S {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::T), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::T {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::U), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::U {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::V), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::V {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::W), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::W {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::X), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::X {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Y), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Y {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Z), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Z {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key1 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key2 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key3), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key3 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key4), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key4 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key5), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key5 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key6), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key6 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key7), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key7 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key8), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key8 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Key9), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Key9 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Escape), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Escape {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F1 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F2 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F3), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F3 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F4), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F4 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F5), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F5 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F6), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F6 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F7), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F7 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F8), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F8 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F9), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F9 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F10), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F10 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F11), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F11 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F12), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F12 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F13), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F13 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F14), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F14 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F15), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F15 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F16), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F16 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F17), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F17 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F18), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F18 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F19), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F19 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F20), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F20 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F21), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F21 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F22), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F22 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F23), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F23 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::F24), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::F24 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Snapshot), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Snapshot {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Scroll), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::ScrollLock {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Pause), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Pause {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Insert), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Insert {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Home), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Home {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Delete), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Delete {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::End), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::End {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::PageDown), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::PageDown {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::PageUp), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::PageUp {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Left), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Left {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Up {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Right), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Right {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Down), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Down {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Back), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Back {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Return), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Enter {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Space {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Compose), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Compose {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Caret), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Caret {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numlock), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numlock {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad0), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad0 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad1 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad2 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad3), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad3 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad4), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad4 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad5), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad5 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad6), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad6 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad7), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad7 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad8), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad8 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Numpad9), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Numpad9 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::AbntC1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::AbntC1 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::AbntC2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::AbntC2 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Add), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Add {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Apostrophe), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Apostrophe {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Apps), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Apps {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::At), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::At {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Ax), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Ax {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Backslash), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Backslash {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Calculator), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Calculator {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Capital), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Capital {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Colon), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Colon {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Comma), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Comma {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Convert), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Convert {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Decimal), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Decimal {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Divide), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Divide {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Equals), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Equals {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Grave), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Grave {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Kana), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Kana {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Kanji), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Kanji {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::LAlt), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::LBracket {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::LControl), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::LShift {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::LWin), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::LWin {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Mail), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Mail {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::MediaSelect), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::MediaSelect {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::MediaStop), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::MediaStop {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Minus), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Minus {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Multiply), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Multiply {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Mute), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Mute {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::MyComputer), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::MyComputer {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NavigateForward), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NavigateForward {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NavigateBackward), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NavigateBackward {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NextTrack), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NextTrack {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NoConvert), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NoConvert {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NumpadComma), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NumpadComma {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NumpadEnter), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NumpadEnter {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::NumpadEquals), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::NumpadEquals {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::OEM102), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::OEM102 {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Period), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Period {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::PlayPause), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::PlayPause {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Power), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Power {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::PrevTrack), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::PrevTrack {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::RAlt), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::RAlt {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::RBracket), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::RBracket {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::RControl), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::RControl {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::RShift), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::RShift {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::RWin), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::RWin {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Semicolon), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Semicolon {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Slash), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Slash {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Sleep), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Sleep {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Stop), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Stop {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Subtract), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Subtract {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Sysrq), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Sysrq {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Tab), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Tab {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Underline), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Underline {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Unlabeled), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Unlabeled {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::VolumeDown), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::VolumeDown {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::VolumeUp), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::VolumeUp {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Wake), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Wake {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebBack), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebBack {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebFavorites), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebFavorites {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebForward), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebForward {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebHome), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebHome {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebRefresh), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebRefresh {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebSearch), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebSearch {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::WebStop), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::WebStop {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Yen), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Yen {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Copy), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Copy {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Paste), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Paste {
                                state: State::Released,
                            })
                        }
                        (Some(VirtualKeyCode::Cut), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyEvent::Cut {
                                state: State::Released,
                            })
                        }
                        _ => NazarustEvent::Unknown,
                    }
                }
            },
            _ => NazarustEvent::Unknown,
        },
        _ => NazarustEvent::Unknown,
    }
}
