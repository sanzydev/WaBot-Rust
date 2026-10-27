#[path = "../lib/mod.rs"]
pub mod lib;

#[path = "../plugins/mod.rs"]
pub mod plugins;

mod bot;
mod commands;
mod config;
mod handler;
mod pairing;

use bot::BotEngine;
use config::Config;
use plugins::PluginManager;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::load()?;
    let plugin_manager = Arc::new(PluginManager::new());
    let engine = BotEngine::new(config, plugin_manager);
    engine.launch().await?;
    Ok(())
}
