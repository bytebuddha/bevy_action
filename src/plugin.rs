use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::app::{Plugin, AppBuilder};
use bevy::input::mouse::MouseMotion;
use crate::{ Event, Axis, Button, MouseAxis, ConfigActions, Actions, ActionsConfig, ConfigActionsLoader };

pub struct ConfigActionsPlugin<A: ConfigActions>(std::marker::PhantomData<A>);

impl <A: ConfigActions>Default for ConfigActionsPlugin<A> {
    fn default() -> ConfigActionsPlugin<A> {
        ConfigActionsPlugin(Default::default())
    }
}

impl <A: ConfigActions>Plugin for ConfigActionsPlugin<A> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ActionsConfig<A>>()
            .add_asset_loader(ConfigActionsLoader::<A>::default())
            .init_resource::<Actions<A>>()
            .add_startup_system(initialize::<A>.system())
            .add_system(handle_keyboard_button_input::<A>.system())
            .add_system(handle_mouse_button_input::<A>.system())
            .add_system(handle_mouse_axis_input::<A>.system())
            .add_system(handle_gamepad_button_input::<A>.system())
            .add_system(handle_gamepad_axis_input::<A>.system());
    }
}

fn initialize<A: ConfigActions>(
    assets: Res<AssetServer>,
    mut res: ResMut<Actions<A>>,
) {
    res.handle = assets.load(A::PATH);
}

fn handle_keyboard_button_input<A: ConfigActions>(
    input: Res<Input<KeyCode>>,
    mut actions: ResMut<Actions<A>>,
    configs: Res<Assets<ActionsConfig<A>>>,

) {
    if let Some(config) = configs.get(actions.handle.clone()) {
        for (event, action) in config.data.iter() {
            match event {
                Event::JustPressed(btn) => match btn {
                    Button::Keyboard(code) => {
                        if input.just_pressed(*code) {
                            actions.data.insert(*action, None);
                        } else {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        }
                    },
                    _ => {}
                },
                Event::Pressed(btn) => match btn {
                    Button::Keyboard(code) => {
                        if input.pressed(*code) {
                            actions.data.insert(*action, None);
                        } else {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn handle_mouse_button_input<A: ConfigActions>(
    input: Res<Input<MouseButton>>,
    mut actions: ResMut<Actions<A>>,
    configs: Res<Assets<ActionsConfig<A>>>
) {
    if let Some(config) = configs.get(actions.handle.clone()) {
        for (event, action) in config.data.iter() {
            match event {
                Event::JustPressed(btn) => match btn {
                    Button::Mouse(btn) => {
                        if input.just_pressed(*btn) {
                            actions.data.insert(*action, None);
                        } else {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        }
                    },
                    _ => {}
                },
                Event::Pressed(btn) => match btn {
                    Button::Mouse(btn) => {
                        if input.pressed(*btn) {
                            actions.data.insert(*action, None);
                        } else {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn handle_mouse_axis_input<A: ConfigActions>(
    mut actions: ResMut<Actions<A>>,
    mut input: EventReader<MouseMotion>,
    mut configs: ResMut<Assets<ActionsConfig<A>>>
) {
    if let Some(config) = configs.get_mut(actions.handle.clone()) {
        for MouseMotion { delta } in input.iter() {
            if let Some(action) = config.data.get(&Event::Axis(Axis::Mouse(MouseAxis::X))) {
                if delta.x == 0.0 {
                    if actions.data.contains_key(action) {
                        actions.data.remove(action);
                    }
                } else {
                    actions.data.insert(*action, Some(delta.x));
                }
            }
            if let Some(action) = config.data.get(&Event::Axis(Axis::Mouse(MouseAxis::Y))) {
                if delta.y == 0.0 {
                    if actions.data.contains_key(action) {
                        actions.data.remove(action);
                    }
                } else {                    
                    actions.data.insert(*action, Some(delta.y));
                }
            }
        }
    }
}

fn handle_gamepad_button_input<A: ConfigActions>(
    mut actions: ResMut<Actions<A>>,
    input: Res<Input<GamepadButton>>,
    configs: Res<Assets<ActionsConfig<A>>>
) {
    if let Some(config) = configs.get(actions.handle.clone()) {
        for (event, action) in config.data.iter() {
            match event {
                Event::JustPressed(btn) => match btn {
                    Button::Gamepad(id, btn) => {
                        if input.just_pressed(GamepadButton(Gamepad(*id), *btn)) {
                            actions.data.insert(*action, None);
                        } else {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        }
                    },
                    _ => {}
                },
                Event::Pressed(btn) => match btn {
                    Button::Gamepad(id, btn) => {
                        if input.pressed(GamepadButton(Gamepad(*id), *btn)) {
                            actions.data.insert(*action, None);
                        } else {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn handle_gamepad_axis_input<A: ConfigActions>(
    mut actions: ResMut<Actions<A>>,
    mut input: EventReader<GamepadEvent>,
    configs: Res<Assets<ActionsConfig<A>>>
) {
    if let Some(config) = configs.get(actions.handle.clone()) {
        for GamepadEvent(gamepad, event) in input.iter() {
            match event {
                GamepadEventType::AxisChanged(axis, value) => {
                    if let Some(action) = config.data.get(&Event::Axis(Axis::Gamepad(gamepad.0, *axis))) {
                        if *value == 0.0 {
                            if actions.data.contains_key(action) {
                                actions.data.remove(action);
                            }
                        } else {
                            actions.data.insert(*action, Some(*value));
                        }
                    }
                },
                _ => {}
            }
        }
    }
}
