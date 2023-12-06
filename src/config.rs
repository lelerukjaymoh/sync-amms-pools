use anyhow::{anyhow, Result};
use dotenv::dotenv;
use reqwest::Url;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub wss_rpc: Url,
}

impl Config {
    pub async fn read_from_dotenv() -> Result<Self> {
        dotenv().ok();

        let get_env = |var| {
            env::var(var).map_err(|_| anyhow!("Required environment variable \"{}\" not set", var))
        };

        let wss_rpc = get_env("WSS_RPC_URL")?
            .parse()
            .map_err(|_| anyhow!("Invalid WSS_RPC_URL"))?;

        Ok(Self { wss_rpc })
    }
}
