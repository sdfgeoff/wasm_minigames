# Keyboard Input

In the previous example, the keyboard input acts as though the player
holds down any key he presses. This is because HTML/Javascript doesn't
give us a way to query if a key is currently held down - it only gives
is `keydown` and `keyup` events. Back in 
[binding_events](../binding_events/index.md) we just smashed all
the key events into one. It's time to break those into separate function
calls and to maintain state for the keys we are interested in.

Assing the extra binding is a case of modifying the `Core` struct to
separate the bindings:
```
        {
            // keyboard events
            self.canvas.set_tab_index(1); // Canvas elements ignore key events unless they have a tab index
            let anim_app1 = self.app.clone();
            let anim_app2 = self.app.clone();

            let keydown_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                let e: Event = event.clone().dyn_into().unwrap();
                e.stop_propagation();
                e.prevent_default();

                anim_app1.borrow_mut().keydown_event(event);
            }) as Box<dyn FnMut(_)>);
            
            let keyup_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                let e: Event = event.clone().dyn_into().unwrap();
                e.stop_propagation();
                e.prevent_default();

                anim_app2.borrow_mut().keyup_event(event);
            }) as Box<dyn FnMut(_)>);

            self.canvas
                .add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())
                .unwrap();
                
            self.canvas
                .add_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())
                .unwrap();

            keydown_callback.forget();
            keyup_callback.forget();
        }
```

And creating the extra function in our App struct:
```
    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        // Do something
    }
    
    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        // Do something else
    }
```

Now we need to mantain the state. Let's create an enum to represent
the state of the keys and how it transitions between states.
```
impl KeyState {
    fn update(&self) -> KeyState {
        match self {
            KeyState::JustPressed => KeyState::Down,
            KeyState::Down => KeyState::Down,
            KeyState::JustReleased => KeyState::Up,
            KeyState::Up => KeyState::Up,
        }
    }
    
    fn active(&self) -> bool {
        match self {
            KeyState::JustPressed => true,
            KeyState::Down => true,
            KeyState::JustReleased => false,
            KeyState::Up => false,
        }
    }
}
```
So the idea is that the JS events set the `KeyState` into
`JustPressed` or `JustReleased`, and then on the subsequent frames
it is in the state `Down` or `Up`. Code can either query the edge event
by looking at the value of the KeyState directly, or can use the "active"
function to determine if the key is in a "downy" state.

And now create a struct to store the state for each key we're interested in:

```
struct KeyMap {
    forwards: KeyState,
    backwards: KeyState,
    turn_left: KeyState,
    turn_right: KeyState,
}

impl KeyMap {
    fn new() -> Self {
        Self {
            forwards: KeyState::Up,
            backwards: KeyState::Up,
            turn_left: KeyState::Up,
            turn_right: KeyState::Up,
        }
    }
    
    
    fn update(&mut self) {
        self.forwards = self.forwards.update();
        self.backwards = self.backwards.update();
        self.turn_left = self.turn_left.update();
        self.turn_right = self.turn_right.update();
    }
    
    
    fn set_state_from_str(&mut self, code: &str, new_state: KeyState) {
        match code {
            "KeyW" => {self.forwards = new_state},
            "KeyS" => {self.backwards = new_state},
            "KeyA" => {self.turn_left = new_state},
            "KeyD" => {self.turn_right = new_state},
            _ => ()
        };
    }
}
```
On the `keyup` and `keydown` events the function `set_state_from_str` will
be called, and on every action frame, `update` will be called.

One final thing and that is that the `keydown` event continues to fire when
held down, so the contents of our keydown and keyup functions should be:
```
    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        if !event.repeat() {
            self.key_map.set_state_from_str(&event.code(), KeyState::JustPressed);
        }
    }
    
    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        self.key_map.set_state_from_str(&event.code(), KeyState::JustReleased);
    }
```

Now we can map the state of the `key_map` to the player in our animation frame
callback:
```
let player_ship = &mut self.ship_entities[0];
player_ship.linear_thrust = 0.0;
player_ship.angular_thrust = 0.0;
if self.key_map.forwards.active() {
    player_ship.linear_thrust += 1.0
}
if self.key_map.backwards.active() {
    player_ship.linear_thrust -= 1.0
}
if self.key_map.turn_left.active() {
    player_ship.angular_thrust += 1.0
}
if self.key_map.turn_right.active() {
    player_ship.angular_thrust -= 1.0
}
self.key_map.update();
```

And the result is:
<canvas id="swoop_ship_keyboard_input"></canvas>

Why implement it all this way? Why not convert the key string to an 
enum then use a hashmap to store key state, and make the KeyMap more 
generic? The same reason I didn't factor out generic "sprite drawing 
code" - I'm not trying to make a game engine here, and this is the 
simplest way to get the job done.
