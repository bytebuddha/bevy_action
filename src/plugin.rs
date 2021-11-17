use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::app::{Plugin, AppBuilder};
use crate::{ Event, Button, ConfigActions, Actions, ActionsConfig, ConfigActionsLoader };

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
            .add_system(handle_keyboard_input::<A>.system())
            .add_system(handle_mouse_input::<A>.system())
            .add_system(handle_gamepad_input::<A>.system());
    }
}

fn initialize<A: ConfigActions>(
    assets: Res<AssetServer>,
    mut res: ResMut<Actions<A>>,
) {
    res.handle = assets.load(A::PATH);
}

fn handle_keyboard_input<A: ConfigActions>(
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
                }
            }
        }
    }
}

fn handle_mouse_input<A: ConfigActions>(
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
                }
            }
        }
    }
}

fn handle_gamepad_input<A: ConfigActions>(
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
                }
            }
        }
    }
}
