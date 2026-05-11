use winit::event::MouseButton;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseButtonState {
    Pressed,
    Released,
}

#[derive(Clone, Debug)]
pub enum InputEvent {
    MouseMove(MousePos),
    MouseDown(MouseButton, MousePos),
    MouseUp(MouseButton, MousePos),
    MouseWheel(f64, f64),
    KeyDown(u32),
    KeyUp(u32),
}

pub struct InputState {
    pub mouse_pos: Option<MousePos>,
    pub mouse_buttons: [bool; 8],
    pub keys: [bool; 512],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            mouse_buttons: [false; 8],
            keys: [false; 512],
        }
    }

    pub fn is_key_pressed(&self, key: u32) -> bool {
        if (key as usize) < self.keys.len() {
            self.keys[key as usize]
        } else {
            false
        }
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        let idx = match button {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
            MouseButton::Other(_) => return false,
        };
        if (idx as usize) < self.mouse_buttons.len() {
            self.mouse_buttons[idx as usize]
        } else {
            false
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

