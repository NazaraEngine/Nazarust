use crate::events::{
    KeyCode, KeyState, MouseButton as NazarustMouseButton, MouseEvent, State,
    WindowEvent as NazarustWindowEvent,
};
use winit::event::{
    ElementState, Event as WinitEvent, KeyboardInput, MouseButton as WinitMouseButton,
    VirtualKeyCode, WindowEvent,
};
pub enum NazarustEvent {
    KeyEvent(KeyState),
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
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::A))
                        }
                        (Some(VirtualKeyCode::B), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::B))
                        }
                        (Some(VirtualKeyCode::C), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::C))
                        }
                        (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::D))
                        }
                        (Some(VirtualKeyCode::E), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::E))
                        }
                        (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F))
                        }
                        (Some(VirtualKeyCode::G), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::G))
                        }
                        (Some(VirtualKeyCode::H), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::H))
                        }
                        (Some(VirtualKeyCode::I), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::I))
                        }
                        (Some(VirtualKeyCode::J), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::J))
                        }
                        (Some(VirtualKeyCode::K), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::K))
                        }
                        (Some(VirtualKeyCode::L), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::L))
                        }
                        (Some(VirtualKeyCode::M), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::M))
                        }
                        (Some(VirtualKeyCode::N), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::N))
                        }
                        (Some(VirtualKeyCode::O), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::O))
                        }
                        (Some(VirtualKeyCode::P), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::P))
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Q))
                        }
                        (Some(VirtualKeyCode::R), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::R))
                        }
                        (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::S))
                        }
                        (Some(VirtualKeyCode::T), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::T))
                        }
                        (Some(VirtualKeyCode::U), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::U))
                        }
                        (Some(VirtualKeyCode::V), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::V))
                        }
                        (Some(VirtualKeyCode::W), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::W))
                        }
                        (Some(VirtualKeyCode::X), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::X))
                        }
                        (Some(VirtualKeyCode::Y), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Y))
                        }
                        (Some(VirtualKeyCode::Z), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Z))
                        }
                        (Some(VirtualKeyCode::Key1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key1))
                        }
                        (Some(VirtualKeyCode::Key2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key2))
                        }
                        (Some(VirtualKeyCode::Key3), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key3))
                        }
                        (Some(VirtualKeyCode::Key4), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key4))
                        }
                        (Some(VirtualKeyCode::Key5), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key5))
                        }
                        (Some(VirtualKeyCode::Key6), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key6))
                        }
                        (Some(VirtualKeyCode::Key7), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key7))
                        }
                        (Some(VirtualKeyCode::Key8), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key8))
                        }
                        (Some(VirtualKeyCode::Key9), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Key9))
                        }
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Escape))
                        }
                        (Some(VirtualKeyCode::F1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F1))
                        }
                        (Some(VirtualKeyCode::F2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F2))
                        }
                        (Some(VirtualKeyCode::F3), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F3))
                        }
                        (Some(VirtualKeyCode::F4), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F4))
                        }
                        (Some(VirtualKeyCode::F5), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F5))
                        }
                        (Some(VirtualKeyCode::F6), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F6))
                        }
                        (Some(VirtualKeyCode::F7), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F7))
                        }
                        (Some(VirtualKeyCode::F8), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F8))
                        }
                        (Some(VirtualKeyCode::F9), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F9))
                        }
                        (Some(VirtualKeyCode::F10), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F10))
                        }
                        (Some(VirtualKeyCode::F11), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F11))
                        }
                        (Some(VirtualKeyCode::F12), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F12))
                        }
                        (Some(VirtualKeyCode::F13), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F13))
                        }
                        (Some(VirtualKeyCode::F14), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F14))
                        }
                        (Some(VirtualKeyCode::F15), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F15))
                        }
                        (Some(VirtualKeyCode::F16), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F16))
                        }
                        (Some(VirtualKeyCode::F17), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F17))
                        }
                        (Some(VirtualKeyCode::F18), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F18))
                        }
                        (Some(VirtualKeyCode::F19), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F19))
                        }
                        (Some(VirtualKeyCode::F20), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F20))
                        }
                        (Some(VirtualKeyCode::F21), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F21))
                        }
                        (Some(VirtualKeyCode::F22), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F22))
                        }
                        (Some(VirtualKeyCode::F23), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F23))
                        }
                        (Some(VirtualKeyCode::F24), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::F24))
                        }
                        (Some(VirtualKeyCode::Snapshot), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Snapshot))
                        }
                        (Some(VirtualKeyCode::Scroll), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::ScrollLock))
                        }
                        (Some(VirtualKeyCode::Pause), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Pause))
                        }
                        (Some(VirtualKeyCode::Insert), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Insert))
                        }
                        (Some(VirtualKeyCode::Home), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Home))
                        }
                        (Some(VirtualKeyCode::Delete), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Delete))
                        }
                        (Some(VirtualKeyCode::End), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::End))
                        }
                        (Some(VirtualKeyCode::PageDown), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::PageDown))
                        }
                        (Some(VirtualKeyCode::PageUp), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::PageUp))
                        }
                        (Some(VirtualKeyCode::Left), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Left))
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Up))
                        }
                        (Some(VirtualKeyCode::Right), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Right))
                        }
                        (Some(VirtualKeyCode::Down), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Down))
                        }
                        (Some(VirtualKeyCode::Back), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Back))
                        }
                        (Some(VirtualKeyCode::Return), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Enter))
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Space))
                        }
                        (Some(VirtualKeyCode::Compose), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Compose))
                        }
                        (Some(VirtualKeyCode::Caret), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Caret))
                        }
                        (Some(VirtualKeyCode::Numlock), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numlock))
                        }
                        (Some(VirtualKeyCode::Numpad0), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad0))
                        }
                        (Some(VirtualKeyCode::Numpad1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad1))
                        }
                        (Some(VirtualKeyCode::Numpad2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad2))
                        }
                        (Some(VirtualKeyCode::Numpad3), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad3))
                        }
                        (Some(VirtualKeyCode::Numpad4), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad4))
                        }
                        (Some(VirtualKeyCode::Numpad5), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad5))
                        }
                        (Some(VirtualKeyCode::Numpad6), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad6))
                        }
                        (Some(VirtualKeyCode::Numpad7), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad7))
                        }
                        (Some(VirtualKeyCode::Numpad8), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad8))
                        }
                        (Some(VirtualKeyCode::Numpad9), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Numpad9))
                        }
                        (Some(VirtualKeyCode::AbntC1), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::AbntC1))
                        }
                        (Some(VirtualKeyCode::AbntC2), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::AbntC2))
                        }
                        (Some(VirtualKeyCode::Add), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Add))
                        }
                        (Some(VirtualKeyCode::Apostrophe), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Apostrophe))
                        }
                        (Some(VirtualKeyCode::Apps), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Apps))
                        }
                        (Some(VirtualKeyCode::At), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::At))
                        }
                        (Some(VirtualKeyCode::Ax), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Ax))
                        }
                        (Some(VirtualKeyCode::Backslash), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Backslash))
                        }
                        (Some(VirtualKeyCode::Calculator), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Calculator))
                        }
                        (Some(VirtualKeyCode::Capital), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Capital))
                        }
                        (Some(VirtualKeyCode::Colon), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Colon))
                        }
                        (Some(VirtualKeyCode::Comma), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Comma))
                        }
                        (Some(VirtualKeyCode::Convert), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Convert))
                        }
                        (Some(VirtualKeyCode::Decimal), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Decimal))
                        }
                        (Some(VirtualKeyCode::Divide), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Divide))
                        }
                        (Some(VirtualKeyCode::Equals), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Equals))
                        }
                        (Some(VirtualKeyCode::Grave), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Grave))
                        }
                        (Some(VirtualKeyCode::Kana), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Kana))
                        }
                        (Some(VirtualKeyCode::Kanji), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Kanji))
                        }
                        (Some(VirtualKeyCode::LAlt), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::LBracket))
                        }
                        (Some(VirtualKeyCode::LControl), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::LShift))
                        }
                        (Some(VirtualKeyCode::LWin), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::LWin))
                        }
                        (Some(VirtualKeyCode::Mail), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Mail))
                        }
                        (Some(VirtualKeyCode::MediaSelect), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::MediaSelect))
                        }
                        (Some(VirtualKeyCode::MediaStop), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::MediaStop))
                        }
                        (Some(VirtualKeyCode::Minus), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Minus))
                        }
                        (Some(VirtualKeyCode::Multiply), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Multiply))
                        }
                        (Some(VirtualKeyCode::Mute), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Mute))
                        }
                        (Some(VirtualKeyCode::MyComputer), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::MyComputer))
                        }
                        (Some(VirtualKeyCode::NavigateForward), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NavigateForward))
                        }
                        (Some(VirtualKeyCode::NavigateBackward), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NavigateBackward))
                        }
                        (Some(VirtualKeyCode::NextTrack), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NextTrack))
                        }
                        (Some(VirtualKeyCode::NoConvert), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NoConvert))
                        }
                        (Some(VirtualKeyCode::NumpadComma), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NumpadComma))
                        }
                        (Some(VirtualKeyCode::NumpadEnter), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NumpadEnter))
                        }
                        (Some(VirtualKeyCode::NumpadEquals), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::NumpadEquals))
                        }
                        (Some(VirtualKeyCode::OEM102), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::OEM102))
                        }
                        (Some(VirtualKeyCode::Period), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Period))
                        }
                        (Some(VirtualKeyCode::PlayPause), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::PlayPause))
                        }
                        (Some(VirtualKeyCode::Power), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Power))
                        }
                        (Some(VirtualKeyCode::PrevTrack), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::PrevTrack))
                        }
                        (Some(VirtualKeyCode::RAlt), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::RAlt))
                        }
                        (Some(VirtualKeyCode::RBracket), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::RBracket))
                        }
                        (Some(VirtualKeyCode::RControl), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::RControl))
                        }
                        (Some(VirtualKeyCode::RShift), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::RShift))
                        }
                        (Some(VirtualKeyCode::RWin), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::RWin))
                        }
                        (Some(VirtualKeyCode::Semicolon), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Semicolon))
                        }
                        (Some(VirtualKeyCode::Slash), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Slash))
                        }
                        (Some(VirtualKeyCode::Sleep), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Sleep))
                        }
                        (Some(VirtualKeyCode::Stop), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Stop))
                        }
                        (Some(VirtualKeyCode::Subtract), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Subtract))
                        }
                        (Some(VirtualKeyCode::Sysrq), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Sysrq))
                        }
                        (Some(VirtualKeyCode::Tab), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Tab))
                        }
                        (Some(VirtualKeyCode::Underline), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Underline))
                        }
                        (Some(VirtualKeyCode::Unlabeled), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Unlabeled))
                        }
                        (Some(VirtualKeyCode::VolumeDown), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::VolumeDown))
                        }
                        (Some(VirtualKeyCode::VolumeUp), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::VolumeUp))
                        }
                        (Some(VirtualKeyCode::Wake), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Wake))
                        }
                        (Some(VirtualKeyCode::WebBack), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebBack))
                        }
                        (Some(VirtualKeyCode::WebFavorites), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebFavorites))
                        }
                        (Some(VirtualKeyCode::WebForward), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebForward))
                        }
                        (Some(VirtualKeyCode::WebHome), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebHome))
                        }
                        (Some(VirtualKeyCode::WebRefresh), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebRefresh))
                        }
                        (Some(VirtualKeyCode::WebSearch), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebSearch))
                        }
                        (Some(VirtualKeyCode::WebStop), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::WebStop))
                        }
                        (Some(VirtualKeyCode::Yen), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Yen))
                        }
                        (Some(VirtualKeyCode::Copy), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Copy))
                        }
                        (Some(VirtualKeyCode::Paste), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Paste))
                        }
                        (Some(VirtualKeyCode::Cut), ElementState::Pressed) => {
                            NazarustEvent::KeyEvent(KeyState::Pressed(KeyCode::Cut))
                        }
                        (Some(VirtualKeyCode::A), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::A))
                        }
                        (Some(VirtualKeyCode::B), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::B))
                        }
                        (Some(VirtualKeyCode::C), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::C))
                        }
                        (Some(VirtualKeyCode::D), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::D))
                        }
                        (Some(VirtualKeyCode::E), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::E))
                        }
                        (Some(VirtualKeyCode::F), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F))
                        }
                        (Some(VirtualKeyCode::G), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::G))
                        }
                        (Some(VirtualKeyCode::H), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::H))
                        }
                        (Some(VirtualKeyCode::I), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::I))
                        }
                        (Some(VirtualKeyCode::J), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::J))
                        }
                        (Some(VirtualKeyCode::K), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::K))
                        }
                        (Some(VirtualKeyCode::L), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::L))
                        }
                        (Some(VirtualKeyCode::M), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::M))
                        }
                        (Some(VirtualKeyCode::N), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::N))
                        }
                        (Some(VirtualKeyCode::O), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::O))
                        }
                        (Some(VirtualKeyCode::P), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::P))
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Q))
                        }
                        (Some(VirtualKeyCode::R), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::R))
                        }
                        (Some(VirtualKeyCode::S), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::S))
                        }
                        (Some(VirtualKeyCode::T), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::T))
                        }
                        (Some(VirtualKeyCode::U), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::U))
                        }
                        (Some(VirtualKeyCode::V), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::V))
                        }
                        (Some(VirtualKeyCode::W), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::W))
                        }
                        (Some(VirtualKeyCode::X), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::X))
                        }
                        (Some(VirtualKeyCode::Y), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Y))
                        }
                        (Some(VirtualKeyCode::Z), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Z))
                        }
                        (Some(VirtualKeyCode::Key1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key1))
                        }
                        (Some(VirtualKeyCode::Key2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key2))
                        }
                        (Some(VirtualKeyCode::Key3), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key3))
                        }
                        (Some(VirtualKeyCode::Key4), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key4))
                        }
                        (Some(VirtualKeyCode::Key5), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key5))
                        }
                        (Some(VirtualKeyCode::Key6), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key6))
                        }
                        (Some(VirtualKeyCode::Key7), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key7))
                        }
                        (Some(VirtualKeyCode::Key8), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key8))
                        }
                        (Some(VirtualKeyCode::Key9), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Key9))
                        }
                        (Some(VirtualKeyCode::Escape), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Escape))
                        }
                        (Some(VirtualKeyCode::F1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F1))
                        }
                        (Some(VirtualKeyCode::F2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F2))
                        }
                        (Some(VirtualKeyCode::F3), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F3))
                        }
                        (Some(VirtualKeyCode::F4), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F4))
                        }
                        (Some(VirtualKeyCode::F5), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F5))
                        }
                        (Some(VirtualKeyCode::F6), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F6))
                        }
                        (Some(VirtualKeyCode::F7), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F7))
                        }
                        (Some(VirtualKeyCode::F8), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F8))
                        }
                        (Some(VirtualKeyCode::F9), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F9))
                        }
                        (Some(VirtualKeyCode::F10), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F10))
                        }
                        (Some(VirtualKeyCode::F11), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F11))
                        }
                        (Some(VirtualKeyCode::F12), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F12))
                        }
                        (Some(VirtualKeyCode::F13), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F13))
                        }
                        (Some(VirtualKeyCode::F14), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F14))
                        }
                        (Some(VirtualKeyCode::F15), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F15))
                        }
                        (Some(VirtualKeyCode::F16), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F16))
                        }
                        (Some(VirtualKeyCode::F17), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F17))
                        }
                        (Some(VirtualKeyCode::F18), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F18))
                        }
                        (Some(VirtualKeyCode::F19), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F19))
                        }
                        (Some(VirtualKeyCode::F20), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F20))
                        }
                        (Some(VirtualKeyCode::F21), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F21))
                        }
                        (Some(VirtualKeyCode::F22), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F22))
                        }
                        (Some(VirtualKeyCode::F23), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F23))
                        }
                        (Some(VirtualKeyCode::F24), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::F24))
                        }
                        (Some(VirtualKeyCode::Snapshot), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Snapshot))
                        }
                        (Some(VirtualKeyCode::Scroll), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::ScrollLock))
                        }
                        (Some(VirtualKeyCode::Pause), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Pause))
                        }
                        (Some(VirtualKeyCode::Insert), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Insert))
                        }
                        (Some(VirtualKeyCode::Home), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Home))
                        }
                        (Some(VirtualKeyCode::Delete), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Delete))
                        }
                        (Some(VirtualKeyCode::End), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::End))
                        }
                        (Some(VirtualKeyCode::PageDown), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::PageDown))
                        }
                        (Some(VirtualKeyCode::PageUp), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::PageUp))
                        }
                        (Some(VirtualKeyCode::Left), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Left))
                        }
                        (Some(VirtualKeyCode::Up), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Up))
                        }
                        (Some(VirtualKeyCode::Right), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Right))
                        }
                        (Some(VirtualKeyCode::Down), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Down))
                        }
                        (Some(VirtualKeyCode::Back), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Back))
                        }
                        (Some(VirtualKeyCode::Return), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Enter))
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Space))
                        }
                        (Some(VirtualKeyCode::Compose), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Compose))
                        }
                        (Some(VirtualKeyCode::Caret), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Caret))
                        }
                        (Some(VirtualKeyCode::Numlock), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numlock))
                        }
                        (Some(VirtualKeyCode::Numpad0), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad0))
                        }
                        (Some(VirtualKeyCode::Numpad1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad1))
                        }
                        (Some(VirtualKeyCode::Numpad2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad2))
                        }
                        (Some(VirtualKeyCode::Numpad3), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad3))
                        }
                        (Some(VirtualKeyCode::Numpad4), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad4))
                        }
                        (Some(VirtualKeyCode::Numpad5), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad5))
                        }
                        (Some(VirtualKeyCode::Numpad6), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad6))
                        }
                        (Some(VirtualKeyCode::Numpad7), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad7))
                        }
                        (Some(VirtualKeyCode::Numpad8), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad8))
                        }
                        (Some(VirtualKeyCode::Numpad9), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Numpad9))
                        }
                        (Some(VirtualKeyCode::AbntC1), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::AbntC1))
                        }
                        (Some(VirtualKeyCode::AbntC2), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::AbntC2))
                        }
                        (Some(VirtualKeyCode::Add), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Add))
                        }
                        (Some(VirtualKeyCode::Apostrophe), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Apostrophe))
                        }
                        (Some(VirtualKeyCode::Apps), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Apps))
                        }
                        (Some(VirtualKeyCode::At), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::At))
                        }
                        (Some(VirtualKeyCode::Ax), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Ax))
                        }
                        (Some(VirtualKeyCode::Backslash), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Backslash))
                        }
                        (Some(VirtualKeyCode::Calculator), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Calculator))
                        }
                        (Some(VirtualKeyCode::Capital), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Capital))
                        }
                        (Some(VirtualKeyCode::Colon), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Colon))
                        }
                        (Some(VirtualKeyCode::Comma), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Comma))
                        }
                        (Some(VirtualKeyCode::Convert), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Convert))
                        }
                        (Some(VirtualKeyCode::Decimal), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Decimal))
                        }
                        (Some(VirtualKeyCode::Divide), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Divide))
                        }
                        (Some(VirtualKeyCode::Equals), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Equals))
                        }
                        (Some(VirtualKeyCode::Grave), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Grave))
                        }
                        (Some(VirtualKeyCode::Kana), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Kana))
                        }
                        (Some(VirtualKeyCode::Kanji), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Kanji))
                        }
                        (Some(VirtualKeyCode::LAlt), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::LBracket))
                        }
                        (Some(VirtualKeyCode::LControl), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::LShift))
                        }
                        (Some(VirtualKeyCode::LWin), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::LWin))
                        }
                        (Some(VirtualKeyCode::Mail), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Mail))
                        }
                        (Some(VirtualKeyCode::MediaSelect), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::MediaSelect))
                        }
                        (Some(VirtualKeyCode::MediaStop), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::MediaStop))
                        }
                        (Some(VirtualKeyCode::Minus), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Minus))
                        }
                        (Some(VirtualKeyCode::Multiply), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Multiply))
                        }
                        (Some(VirtualKeyCode::Mute), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Mute))
                        }
                        (Some(VirtualKeyCode::MyComputer), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::MyComputer))
                        }
                        (Some(VirtualKeyCode::NavigateForward), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NavigateForward))
                        }
                        (Some(VirtualKeyCode::NavigateBackward), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NavigateBackward))
                        }
                        (Some(VirtualKeyCode::NextTrack), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NextTrack))
                        }
                        (Some(VirtualKeyCode::NoConvert), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NoConvert))
                        }
                        (Some(VirtualKeyCode::NumpadComma), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NumpadComma))
                        }
                        (Some(VirtualKeyCode::NumpadEnter), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NumpadEnter))
                        }
                        (Some(VirtualKeyCode::NumpadEquals), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::NumpadEquals))
                        }
                        (Some(VirtualKeyCode::OEM102), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::OEM102))
                        }
                        (Some(VirtualKeyCode::Period), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Period))
                        }
                        (Some(VirtualKeyCode::PlayPause), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::PlayPause))
                        }
                        (Some(VirtualKeyCode::Power), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Power))
                        }
                        (Some(VirtualKeyCode::PrevTrack), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::PrevTrack))
                        }
                        (Some(VirtualKeyCode::RAlt), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::RAlt))
                        }
                        (Some(VirtualKeyCode::RBracket), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::RBracket))
                        }
                        (Some(VirtualKeyCode::RControl), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::RControl))
                        }
                        (Some(VirtualKeyCode::RShift), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::RShift))
                        }
                        (Some(VirtualKeyCode::RWin), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::RWin))
                        }
                        (Some(VirtualKeyCode::Semicolon), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Semicolon))
                        }
                        (Some(VirtualKeyCode::Slash), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Slash))
                        }
                        (Some(VirtualKeyCode::Sleep), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Sleep))
                        }
                        (Some(VirtualKeyCode::Stop), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Stop))
                        }
                        (Some(VirtualKeyCode::Subtract), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Subtract))
                        }
                        (Some(VirtualKeyCode::Sysrq), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Sysrq))
                        }
                        (Some(VirtualKeyCode::Tab), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Tab))
                        }
                        (Some(VirtualKeyCode::Underline), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Underline))
                        }
                        (Some(VirtualKeyCode::Unlabeled), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Unlabeled))
                        }
                        (Some(VirtualKeyCode::VolumeDown), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::VolumeDown))
                        }
                        (Some(VirtualKeyCode::VolumeUp), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::VolumeUp))
                        }
                        (Some(VirtualKeyCode::Wake), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Wake))
                        }
                        (Some(VirtualKeyCode::WebBack), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebBack))
                        }
                        (Some(VirtualKeyCode::WebFavorites), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebFavorites))
                        }
                        (Some(VirtualKeyCode::WebForward), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebForward))
                        }
                        (Some(VirtualKeyCode::WebHome), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebHome))
                        }
                        (Some(VirtualKeyCode::WebRefresh), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebRefresh))
                        }
                        (Some(VirtualKeyCode::WebSearch), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebSearch))
                        }
                        (Some(VirtualKeyCode::WebStop), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::WebStop))
                        }
                        (Some(VirtualKeyCode::Yen), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Yen))
                        }
                        (Some(VirtualKeyCode::Copy), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Copy))
                        }
                        (Some(VirtualKeyCode::Paste), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Paste))
                        }
                        (Some(VirtualKeyCode::Cut), ElementState::Released) => {
                            NazarustEvent::KeyEvent(KeyState::Released(KeyCode::Cut))
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
