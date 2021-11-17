use bevy::prelude::*;
use bevy_actions::*;
use bevy::utils::HashMap;

#[config_actions(
    file = "simple.ron",
    uuid = "1c3bed05-a109-4ec4-8e63-0e20a27313ee"
)]
pub enum SimpleActions {
    #[Pressed(Keyboard(Q))]
    ActionQ,
    #[Pressed(Keyboard(W))]
    ActionW,
    #[Pressed(Keyboard(E))]
    ActionE,
    #[Pressed(Keyboard(R))]
    ActionR,
    #[Pressed(Keyboard(T))]
    ActionT,
    #[Pressed(Keyboard(Y))]
    ActionY,
    #[Pressed(Keyboard(U))]
    ActionU,
    #[Pressed(Keyboard(I))]
    ActionI,
    #[Pressed(Keyboard(O))]
    ActionO,
    #[Pressed(Keyboard(P))]
    ActionP,
    #[Pressed(Mouse(Left))]
    LeftMouse,
    #[JustPressed(Mouse(Right))]
    RightMouse,
    #[JustPressed(Mouse(Middle))]
    MiddleMouse,
    #[JustPressed(Gamepad(0, South))]
    GamepadSouth
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigActionsPlugin::<SimpleActions>::default())
        .add_system(print_action.system())
        .run()
}

fn print_action(
    actions: Res<Actions<SimpleActions>>
) {
    if !actions.data.is_empty() {
        println!("{:?}", actions.data);
    }
}