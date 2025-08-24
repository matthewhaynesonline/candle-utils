// TODO: move this somewhere else?
use candle_core::Result;
use candle_nn::VarBuilder;
use serde::de::DeserializeOwned;

pub mod bert;

///  Generic trait for models that can be loaded
pub trait LoadableModel<C> {
    fn load(vb: VarBuilder, config: &C) -> Result<Self>
    where
        Self: Sized;
}

/// Generic trait for configs that can be deserialized
pub trait ModelConfig: DeserializeOwned {}
