use std::sync::Arc;
use std::time::Instant;
use whatsapp_rust::Client;
use whatsapp_rust::wacore::types::events::{Event, InboundMessage};
use crate::commands::CommandRouter;
use crate::config::Config;
use crate::lib::jid::resolve_to_pn;
use crate::pairing;
use crate::plugins::PluginManager;

#[derive(Clone)]
pub struct AppHandler {
    config: Config,
    plugin_manager: Arc<PluginManager>,
}

impl AppHandler {
    pub fn new(config: Config, plugin_manager: Arc<PluginManager>) -> Self {
        Self {
            config,
            plugin_manager,
        }
    }

    pub async fn handle_event(&self, event: Arc<Event>, client: Arc<Client>) {
        let start_time = Instant::now();

        match &*event {
            Event::Messages(batch) => {
                for InboundMessage { message, info, .. } in batch.iter() {
                    let extracted_text = if let Some(conv) = &message.conversation {
                        Some(conv.as_str())
                    } else if let Some(ext) = message.extended_text_message.as_option() {
                        ext.text.as_deref()
                    } else {
                        None
                    };

                    if let Some(text) = extracted_text {
                        let sender_pn = resolve_to_pn(&client, &info.source.sender).await;
                        let chat_pn = if info.source.chat.is_lid() {
                            resolve_to_pn(&client, &info.source.chat).await
                        } else {
                            info.source.chat.clone()
                        };



                        let stanza_id = info.id.to_string();
                        let is_from_me = info.source.is_from_me;

                        let _ = CommandRouter::dispatch(
                            &client,
                            &self.plugin_manager,
                            chat_pn,
                            sender_pn,
                            stanza_id,
                            is_from_me,
                            text,
                            message,
                            &self.config,
                            start_time,
                        )
                        .await;
                    }
                }
            }
            Event::Connected(_) => {
                println!("Connected");
            }
            Event::PairingCode(code) => {
                pairing::display_pairing_code(code);
            }
            Event::PairingCodeError(err) => {
                pairing::handle_pairing_error(err);
            }
            Event::PairingCodeRefresh(refresh) => {
                pairing::handle_pairing_refresh(refresh);
            }
            Event::PairSuccess(info) => {
                pairing::handle_pair_success(info);
            }
            Event::Disconnected(_) => {
                println!("Disconnected");
            }
            _ => {}
        }
    }
}
