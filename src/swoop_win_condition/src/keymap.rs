#[derive(Debug, PartialEq)]
/// A state machine to represent the state of a key on the keyboard
/// preserving edge-states.
/// Most of the time will transition `JustPressed -> Down -> JustReleased -> Up`
///
pub enum KeyState {
    /// Represents that the key has been pressed since the last
    /// call to update()
    JustPressed,
    /// Represents that the key is held down
    Down,
    /// Represents that the key has been released since the last
    /// call to update()
    JustReleased,
    /// Represents that the key is not held down
    Up,
}

impl KeyState {
    /// Transition from the edge states (`JustPressed`, `JustReleased`) to the steady states (`Up`, `Down)
    pub fn update(&self) -> KeyState {
        match self {
            KeyState::JustPressed => KeyState::Down,
            KeyState::Down => KeyState::Down,
            KeyState::JustReleased => KeyState::Up,
            KeyState::Up => KeyState::Up,
        }
    }

    /// Similar to how is JS an integer is `truthy` this returns if the
    /// key is `downy` - in a state where the player has the key down.
    /// This includes both the edge and steady state.
    pub fn active(&self) -> bool {
        match self {
            KeyState::JustPressed => true,
            KeyState::Down => true,
            KeyState::JustReleased => false,
            KeyState::Up => false,
        }
    }
}

/// Stores the state of the keys that we are interested in for this game
#[derive(Debug)]
pub struct KeyMap {
    pub forwards: KeyState,
    pub backwards: KeyState,
    pub turn_left: KeyState,
    pub turn_right: KeyState,
    pub start_game: KeyState,
}

impl KeyMap {
    pub fn new() -> Self {
        Self {
            forwards: KeyState::Up,
            backwards: KeyState::Up,
            turn_left: KeyState::Up,
            turn_right: KeyState::Up,
            start_game: KeyState::Up,
        }
    }

    /// Progress each keys state machine
    pub fn update(&mut self) {
        self.forwards = self.forwards.update();
        self.backwards = self.backwards.update();
        self.turn_left = self.turn_left.update();
        self.turn_right = self.turn_right.update();
        self.start_game = self.start_game.update();
    }

    /// Force the state of a specific key based on a "key code" string.
    /// This code generally comes from a javascript `KeyboardEvent.code()`
    pub fn set_state_from_str(&mut self, code: &str, new_state: KeyState) {
        match code {
            "KeyW" | "ArrowUp" => self.forwards = new_state,
            "KeyS" | "ArrowDown" => self.backwards = new_state,
            "KeyA" | "ArrowLeft" => self.turn_left = new_state,
            "KeyD" | "ArrowRight" => self.turn_right = new_state,
            "Enter" => self.start_game = new_state,
            _ => (),
        };
    }
}
