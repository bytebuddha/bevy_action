use bevy::utils::{HashMap, BoxedFuture};
use bevy::asset::{LoadedAsset, AssetLoader, LoadContext};

use crate::{Event, ConfigActions, ActionsConfig};

pub struct ConfigActionsLoader<T: ConfigActions>(std::marker::PhantomData<T>);

impl <T: ConfigActions>Default for ConfigActionsLoader<T> {
    fn default() -> ConfigActionsLoader<T> {
        ConfigActionsLoader(Default::default())
    }
}

impl <T: ConfigActions>AssetLoader for ConfigActionsLoader<T> {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let config_asset = ron::de::from_bytes::<HashMap<T, Vec<Event>>>(bytes)?;
            let mut data = T::default_bindings();
            for (action, events) in config_asset.into_iter() {
                events.into_iter().for_each(| event | {
                    data.insert(event, action.clone());
                })
            }
            load_context.set_default_asset(LoadedAsset::new(ActionsConfig { data }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
