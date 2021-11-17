use bevy::asset::Handle;
use bevy::utils::HashMap;

use crate::{ConfigActions, ActionsConfig};

#[derive(Debug)]
pub struct Actions<T: ConfigActions> {
    pub handle: Handle<ActionsConfig<T>>,
    pub data: HashMap<T, Option<f32>>
}

impl <A: ConfigActions>Actions<A> {
    pub fn action(&self, action: A) -> bool {
        self.data.contains_key(&action)
    }

    pub fn value(&self, action: A) -> Option<f32> {
        self.data.get(&action).map(|x|x.map(|x|x)).flatten()
    }
}

impl <T: ConfigActions>Default for Actions<T> {
    fn default() -> Actions<T> {
        Actions {
            handle: Default::default(),
            data: Default::default()
        }
    }
}
