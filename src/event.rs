use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use serde::{ Serialize, Deserialize };
use bevy::input::gamepad::{GamepadButtonType, GamepadAxisType};

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub enum Event {
    Pressed(Button),
    JustPressed(Button),
    Axis(Axis)
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub enum Button {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    Gamepad(usize, GamepadButtonType)
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub enum Axis {
    Mouse(MouseAxis),
    Gamepad(usize, GamepadAxisType)
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub enum MouseAxis {
    X,
    Y
}
