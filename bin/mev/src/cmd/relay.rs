use crate::cmd::config::Config;
use anyhow::{anyhow, Result};
use ethereum_consensus::crypto::SecretKey;
use clap::{Args, Subcommand};
use mev_relay_rs::Service;
use mev_rs::Network;

#[derive(Debug, Args)]
#[clap(about = "🏗 connecting builders to proposers", subcommand_negates_reqs = true)]
pub struct Command {
    #[clap(env, required = true)]
    config_file: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Mock { config_file: String },
}

impl Command {
    pub async fn execute(&self, network: Network) -> Result<()> {
        let (config_file, _mock) = if let Some(subcommand) = self.command.as_ref() {
            match subcommand {
                Commands::Mock { config_file } => (config_file, true),
            }
        } else {
            (self.config_file.as_ref().unwrap(), false)
        };

        let config = Config::from_toml_file(config_file)?;

        if let Some(mut config) = config.relay {
            let key_bytes = [1u8; 32];
            let secret_key = SecretKey::try_from(key_bytes.as_slice()).unwrap();

            config.network = network;
            let service = Service::from(config).spawn(secret_key, None).await?;

            Ok(service.await?)
        } else {
            Err(anyhow!("missing relay config from file provided"))
        }
    }
}
