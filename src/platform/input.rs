use std::collections::HashSet;
use winit::{
    event::MouseButton,
    keyboard::{KeyCode, ModifiersState},
};

#[derive(Clone, Copy, Debug)]
pub struct MousePos {
    pub x: f64,
    pub y: f64,
}

impl MousePos {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub enum InputEvent {
    MouseMove(MousePos),
    MouseDown(MouseButton, MousePos),
    MouseUp(MouseButton, MousePos),
    MouseWheel(f64, f64),
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    Text(String),
}

#[derive(Clone, Debug)]
pub struct InputState {
    pub mouse_pos: Option<MousePos>,
    pub modifiers: ModifiersState,
    mouse_buttons: HashSet<MouseButton>,
    keys: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            modifiers: ModifiersState::empty(),
            mouse_buttons: HashSet::new(),
            keys: HashSet::new(),
        }
    }

    pub fn set_key(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            self.keys.insert(key);
        } else {
            self.keys.remove(&key);
        }
    }

    pub fn set_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if pressed {
            self.mouse_buttons.insert(button);
        } else {
            self.mouse_buttons.remove(&button);
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys.contains(&key)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
