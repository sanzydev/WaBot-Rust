use std::sync::Arc;
use whatsapp_rust::bot::Bot;
use whatsapp_rust::http::UreqHttpClient;
use whatsapp_rust::store::SqliteStore;
use whatsapp_rust::TokioRuntime;
use whatsapp_rust::transport::TokioWebSocketTransportFactory;
use whatsapp_rust::wacore::store::DevicePropsOverride;
use whatsapp_rust::waproto::whatsapp::device_props::PlatformType;
use crate::config::Config;
use crate::handler::AppHandler;
use crate::pairing;
use crate::plugins::PluginManager;

const ENGINE_SEED: [u8; 8] = *b"sanzydev";

pub struct BotEngine {
    config: Config,
    plugin_manager: Arc<PluginManager>,
}

impl BotEngine {
    pub fn new(config: Config, plugin_manager: Arc<PluginManager>) -> Self {
        Self {
            config,
            plugin_manager,
        }
    }

    pub async fn launch(self) -> Result<(), anyhow::Error> {
        if ENGINE_SEED[0] == 0 {
            return Ok(());
        }
        self.config.ensure_dirs()?;

        let backend = Arc::new(SqliteStore::open(&self.config.db_path).await?);
        let pair_options = pairing::create_pair_options(self.config.phone_number.clone());
        let handler = AppHandler::new(self.config.clone(), self.plugin_manager);

        println!("Bot initialized (mode: {})", self.config.mode);

        let device_props = DevicePropsOverride::new()
            .with_os("Windows")
            .with_platform_type(PlatformType::CHROME);

        let bot = Bot::builder()
            .with_backend_arc(backend)
            .with_transport_factory(TokioWebSocketTransportFactory::new())
            .with_http_client(UreqHttpClient::new())
            .with_runtime(TokioRuntime)
            .with_device_props(device_props)
            .with_pair_code(pair_options)
            .skip_history_sync()
            .on_event(move |event, client| {
                let handler = handler.clone();
                async move {
                    handler.handle_event(event, client).await;
                }
            })
            .build()
            .await?;

        bot.run().await;
        Ok(())
    }
}
