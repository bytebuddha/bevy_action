## bevy_action

Input Map plugin for bevy. Uses a procedural macro to specify the events that set off actions. This works by creating a custom asset behind the scenes that represents a config file. The config file and the events specified in using the procedural macro are used to build the `Actions<T>` struct.

```rust

use bevy::prelude::*;
use bevy_actions::*;
use bevy::utils::HashMap;

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
```

Events will also be loaded from a ron config file.
```ron
{
    LeftMouse: [
        Pressed(Mouse(Left))
    ],
    RightMouse: [
        JustPressed(Mouse(Right))
    ],
    GamepadSouth: [
        JustPressed(Gamepad(0, South))
    ]
}
```

## Actions
  Currently for an enum to be considered an `Actions` enum it must implement all of
  `Eq Hash TypeUuid Send Sync Sized Copy Serialize DeserializeOwned 'static`

## Macro
  The attribute macro is the main entry point for using this crate. It implements the `ConfigActions` trait, as well as the dervives the approperiate traits.

  ### Attributes
  - #### Pressed
      The pressed attribute can be used to get a button press, This is a boolean
      value.
  - #### JustPressed
      The JustPressed attribute is used to get whether a button was just pressed the last frame. This is a boolean value.
  - #### Axis
      The Axis attribute is used to pipe the current value from a control axis.
      If the value is `0.0` nothing is used. This is a f64 value.

## TODO
  - [x] Load events statically
  - [x] Load events from config file
  - [ ] Hot reloading the config file
  - [ ] Different config formats YAML/TOML/JSON, etc.

## Alternatives

- [bevy_input_actionmap](https://github.com/lightsoutgames/bevy_input_actionmap)
- [kurinji](https://github.com/PradeepKumarRajamanickam/kurinji)
- [bevy_advanced_input](https://github.com/sadpython/bevy_advanced_input)
