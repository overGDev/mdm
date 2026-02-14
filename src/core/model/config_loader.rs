use crate::core::{error::MDMError, model::MDMConfig};

pub trait ConfigLoader {
    fn load_config(&self) -> Result<MDMConfig, MDMError>;
}