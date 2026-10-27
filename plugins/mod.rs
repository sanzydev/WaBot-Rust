pub mod tools;

use async_trait::async_trait;
use std::sync::Arc;
use whatsapp_rust::Client;
use whatsapp_rust::wacore_binary::Jid;
use whatsapp_rust::waproto::whatsapp as wa;
use crate::config::Config;

#[derive(Clone)]
pub struct PluginContext {
    pub client: Arc<Client>,
    pub chat_jid: Jid,
    pub sender_jid: Jid,
    pub stanza_id: String,
    pub raw_message: wa::Message,
    pub config: Config,
    pub start_time: std::time::Instant,
}

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn commands(&self) -> &[&str];
    async fn run(&self, ctx: &PluginContext) -> Result<(), anyhow::Error>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut manager = Self {
            plugins: Vec::new(),
        };
        manager.register(Box::new(tools::ping::PingPlugin::new()));
        manager
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn find(&self, cmd: &str) -> Option<&(dyn Plugin + 'static)> {
        for plugin in &self.plugins {
            if plugin.commands().contains(&cmd) {
                return Some(plugin.as_ref());
            }
        }
        None
    }

    pub fn list(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
}
