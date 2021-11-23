#![feature(bool_to_option)]

use std::hash::Hash;
use bevy::utils::HashMap;
use bevy::reflect::TypeUuid;
pub use bevy_actions_derive::*;
use serde::{ Serialize, de::DeserializeOwned };

mod event;
pub use self::event::{Button, Event, Axis, MouseAxis};

mod plugin;
pub use self::plugin::ConfigActionsPlugin;

mod loader;
pub use self::loader::ConfigActionsLoader;

mod actions;
pub use self::actions::Actions;

mod config;
pub use self::config::ActionsConfig;

pub trait ConfigActionsRequirements:
    Eq + Hash + TypeUuid +
    Send + Sync + Sized +
    Serialize + DeserializeOwned +
    'static + Copy
{}

impl <T>ConfigActionsRequirements for T
where T:
    Eq + Hash + TypeUuid +
    Send + Sync + Sized + Copy +
    Serialize + DeserializeOwned +
    'static
{}

pub trait ConfigActions: ConfigActionsRequirements {
    const PATH: &'static str;

    fn default_bindings() -> HashMap<Event, Self> {
        Default::default()
    }
}
