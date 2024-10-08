use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub bitcoin_node_url: String,
  pub network: String,
  pub peer_port: u16
}


impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
      let mut s = Config::new();
      s.merge(File::with_name("config/default"))?;
      s.try_into();
  }
}