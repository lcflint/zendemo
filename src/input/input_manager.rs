
use std::collections::HashSet;

use glutin::event::VirtualKeyCode;

//-------------------------

// input manager class
pub struct InputManager {
    captured_keys: HashSet<VirtualKeyCode>
}

impl InputManager {
    // creates a new input manager
    pub fn new() -> InputManager {
        InputManager {
            captured_keys: HashSet::new()
        }
    }

    // registers that a key has been pressed
    pub fn register_key_press(&mut self, key_input: VirtualKeyCode) {
        // inserts into the set
        self.captured_keys.insert(key_input);
    }

    // registers that a key has been released
    pub fn register_key_release(&mut self, key_input: VirtualKeyCode) {
        // gets the key from the set
        let key = self.captured_keys.get(&key_input);

        // if the key is not none, remove the key
        if key != None {
            self.captured_keys.remove(&key_input);
        }
    }

    // polls for whether or not a key is being held down
    pub fn poll_key(&mut self, key_input: VirtualKeyCode) -> bool {
        self.captured_keys.get(&key_input) != None
    }
}