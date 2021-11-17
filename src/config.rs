use bevy::reflect::TypeUuid;
use bevy::utils::{HashMap, Uuid};

use crate::{ConfigActions, Event};

#[derive(Debug)]
pub struct ActionsConfig<A: ConfigActions> {
    pub data: HashMap<Event, A>
}

impl <A: ConfigActions>Default for ActionsConfig<A> {
    fn default() -> ActionsConfig<A> {
        ActionsConfig {
            data: A::default_bindings()
        }
    }
}

impl <A: ConfigActions>ActionsConfig<A> {
    pub fn action(&self, event: Event) -> Option<A> {
        self.data.get(&event).map(|x|(*x))
    }
}

impl <A: ConfigActions>TypeUuid for ActionsConfig<A> {
    const TYPE_UUID: Uuid = A::TYPE_UUID;
}
