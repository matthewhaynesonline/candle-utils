use candle_core::Result;
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};

use crate::candle_transformers_shim::models::{LoadableModel, ModelConfig};

impl ModelConfig for Config {}

impl LoadableModel<Config> for BertModel {
    fn load(vb: VarBuilder, config: &Config) -> Result<Self> {
        Self::load(vb, config)
    }
}
