use crate::lib::message::send_reply;
use crate::plugins::{Plugin, PluginContext};
use async_trait::async_trait;
use sysinfo::{Pid, System};

pub struct PingPlugin;

impl PingPlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Plugin for PingPlugin {
    fn name(&self) -> &str {
        "ping"
    }

    fn description(&self) -> &str {
        "Menampilkan status bot, latency, cpu clock, dan memory"
    }

    fn commands(&self) -> &[&str] {
        &["ping", "speed", "status"]
    }

    async fn run(&self, ctx: &PluginContext) -> Result<(), anyhow::Error> {
        let latency_ms = ctx.start_time.elapsed().as_millis();

        let mut sys = System::new_all();
        sys.refresh_all();

        let pid = Pid::from_u32(std::process::id());
        let process_rss_mb = sys
            .process(pid)
            .map(|p| p.memory() as f64 / (1024.0 * 1024.0))
            .unwrap_or(0.0);

        let total_ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_ram_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

        let cpus = sys.cpus();
        let cpu_clock = cpus.first().map(|c| c.frequency()).unwrap_or(0);
        let core_count = cpus.len();

        let uptime_secs = System::uptime();
        let hours = uptime_secs / 3600;
        let minutes = (uptime_secs % 3600) / 60;
        let seconds = uptime_secs % 60;

        let response = format!(
            "Pong!\n\
            Speed       : {} ms\n\
            CPU Clock   : {} MHz ({} cores)\n\
            Process RSS : {:.2} MB\n\
            System RAM  : {:.2} GB / {:.2} GB\n\
            Uptime      : {}h {}m {}s\n\
            Mode        : {}",
            latency_ms,
            cpu_clock,
            core_count,
            process_rss_mb,
            used_ram_gb,
            total_ram_gb,
            hours,
            minutes,
            seconds,
            if ctx.config.mode == "self" {
                "Self"
            } else {
                "Public"
            }
        );

        send_reply(
            &ctx.client,
            ctx.chat_jid.clone(),
            response,
            ctx.stanza_id.clone(),
            Some(ctx.sender_jid.to_string()),
            ctx.raw_message.clone(),
        )
        .await?;

        Ok(())
    }
}
