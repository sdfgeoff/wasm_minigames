

/// The typical key state transitions are:
/// Released -> JustPressed -> Pressed -> JustReleased -> Released
/// but it can also transition from 
/// JustReleased -> Pressed
#[derive(Clone, Copy)]
enum KeyState {
    Released,
    JustPressed,
    Pressed,    
    JustReleased,
}


impl KeyState {
    pub fn increment(&self, is_down: bool) -> Self {
        match self {
            KeyState::Released => {
                if is_down {
                    KeyState::JustPressed
                } else {
                    KeyState::Released
                }
            }
            KeyState::JustPressed => {
                if is_down {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                }
            }
            KeyState::Pressed => {
                if is_down {
                    KeyState::Pressed
                } else {
                    KeyState::JustReleased
                }
            }
            KeyState::JustReleased => {
                if is_down {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                }
            }
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub enum KeyCode{
    W = 87,
    A = 65,
    S = 83,
    D = 68,
    Q = 81,
    E = 69,
    R = 82,
    F = 70,
    Space = 32,
    Shift = 16,
    Ctrl = 17,
    Alt = 18,
    Escape = 27,
    Left = 37,
    Up = 38,
    Right = 39,
    Down = 40,
}

impl KeyCode {
    pub fn from_js_code(code: &str) -> Option<Self> {
        match code {
            "KeyW" => Some(KeyCode::W),
            "KeyA" => Some(KeyCode::A),
            "KeyS" => Some(KeyCode::S),
            "KeyD" => Some(KeyCode::D),
            "KeyQ" => Some(KeyCode::Q),
            "KeyE" => Some(KeyCode::E),
            "KeyR" => Some(KeyCode::R),
            "KeyF" => Some(KeyCode::F),
            "Space" => Some(KeyCode::Space),
            "ShiftLeft" => Some(KeyCode::Shift),
            "ControlLeft" => Some(KeyCode::Ctrl),
            "AltLeft" => Some(KeyCode::Alt),
            "Escape" => Some(KeyCode::Escape),
            "ArrowLeft" => Some(KeyCode::Left),
            "ArrowUp" => Some(KeyCode::Up),
            "ArrowRight" => Some(KeyCode::Right),
            "ArrowDown" => Some(KeyCode::Down),
            _ => None,
        }
    }
}


pub struct Keyboard {
    key_states: [KeyState; 256],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_states: [KeyState::Released; 256],
        }
    }

    pub fn set_key_state(&mut self, key_code: KeyCode, is_down: bool) {
        self.key_states[key_code as usize] = self.key_states[key_code as usize].increment(is_down);
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        match self.key_states[key_code as usize] {
            KeyState::Pressed | KeyState::JustPressed => true,
            _ => false,
        }
    }

    pub fn is_key_released(&self, key_code: KeyCode) -> bool {
        match self.key_states[key_code as usize] {
            KeyState::Released | KeyState::JustReleased => true,
            _ => false,
        }
    }

    pub fn is_key_just_pressed(&self, key_code: KeyCode) -> bool {
        match self.key_states[key_code as usize] {
            KeyState::JustPressed => true,
            _ => false,
        }
    }

    pub fn is_key_just_released(&self, key_code: KeyCode) -> bool {
        match self.key_states[key_code as usize] {
            KeyState::JustReleased => true,
            _ => false,
        }
    }
}
