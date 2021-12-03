use bevy::prelude::*;
use bevy_actions::*;

#[config_actions(
    file = "simple.ron",
    uuid = "1c3bed05-a109-4ec4-8e63-0e20a27313ee"
)]
pub enum SimpleActions {
    #[Pressed(Keyboard(Q))]
    #[Pressed(Keyboard(W))]
    #[Pressed(Keyboard(E))]
    #[Pressed(Keyboard(R))]
    #[Pressed(Keyboard(T))]
    #[Pressed(Keyboard(Y))]
    Qwerty,
    #[Pressed(Mouse(Left))]
    LeftMouse,
    #[JustPressed(Mouse(Right))]
    RightMouse,
    #[JustPressed(Mouse(Middle))]
    MiddleMouse,
    #[JustPressed(Gamepad(0, South))]
    GamepadSouth,
    #[Axis(Mouse(X))]
    MouseX,
    #[Axis(Mouse(Y))]
    MouseY,
    #[Axis(Gamepad(0, LeftStickX))]
    LeftStickX
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
