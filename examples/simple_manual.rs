use bevy::prelude::*;
use bevy_actions::*;
use bevy::utils::HashMap;
use bevy::reflect::TypeUuid;
use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Serialize, Deserialize, TypeUuid)]
#[uuid = "1c3bed05-a109-4ec4-8e63-0e20a27313ee"]
pub enum SimpleActions {
    ActionQ,
    ActionW,
    ActionE,
    ActionR,
    ActionT,
    ActionY,
    ActionU,
    ActionI,
    ActionO,
    ActionP,
    LeftMouse,
    RightMouse,
    GamepadSouth,
    MouseX,
    MouseY,
    LeftStickX
}

impl ConfigActions for SimpleActions {
    const PATH: &'static str = "simple.ron";

    fn default_bindings() -> HashMap<Event, SimpleActions> {
        let mut map = HashMap::default();
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::Q)), SimpleActions::ActionQ);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::W)), SimpleActions::ActionW);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::E)), SimpleActions::ActionE);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::R)), SimpleActions::ActionR);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::T)), SimpleActions::ActionT);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::Y)), SimpleActions::ActionY);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::U)), SimpleActions::ActionU);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::I)), SimpleActions::ActionI);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::O)), SimpleActions::ActionO);
        map.insert(Event::Pressed(bevy_actions::Button::Keyboard(KeyCode::P)), SimpleActions::ActionP);
        map.insert(Event::Pressed(bevy_actions::Button::Gamepad(0, GamepadButtonType::South)), SimpleActions::GamepadSouth);
        map.insert(Event::Axis(bevy_actions::Axis::Mouse(MouseAxis::X)), SimpleActions::MouseX);
        map.insert(Event::Axis(bevy_actions::Axis::Mouse(MouseAxis::Y)), SimpleActions::MouseY);
        map.insert(Event::Axis(bevy_actions::Axis::Gamepad(0, GamepadAxisType::LeftStickX)), SimpleActions::LeftStickX);
        map
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigActionsPlugin::<SimpleActions>::default())
        .add_system(print_action)
        .run()
}

fn print_action(
    actions: Res<Actions<SimpleActions>>
) {
    if !actions.data.is_empty() {
        println!("{:?}", actions.data);
    }
}
