use std::sync::Arc;
use std::time::Instant;
use crate::config::Config;
use crate::lib::message::{send_reply, send_text};
use crate::plugins::{PluginContext, PluginManager};
use whatsapp_rust::Client;
use whatsapp_rust::wacore_binary::Jid;
use whatsapp_rust::waproto::whatsapp as wa;

pub struct CommandRouter;

impl CommandRouter {
    pub async fn dispatch(
        client: &Arc<Client>,
        plugin_manager: &PluginManager,
        chat_jid: Jid,
        sender_jid: Jid,
        stanza_id: String,
        is_from_me: bool,
        text: &str,
        raw_message: &wa::Message,
        config: &Config,
        start_time: Instant,
    ) -> Result<(), anyhow::Error> {
        let trimmed = text.trim();

        if trimmed.starts_with('$') {
            if !config.has_permission(&sender_jid.to_string(), is_from_me) {
                return Ok(());
            }

            let cmd_str = trimmed[1..].trim();
            if cmd_str.is_empty() {
                return Ok(());
            }

            let cmd_lower = cmd_str.to_lowercase();
            let is_ls = cmd_lower == "ls" || cmd_lower.starts_with("ls ");

            let response = if is_ls {
                let parts: Vec<&str> = cmd_str.split_whitespace().collect();
                let mut path = ".";
                let mut is_long = false;
                for part in &parts[1..] {
                    if part.starts_with('-') {
                        if part.contains('l') {
                            is_long = true;
                        }
                    } else {
                        path = part;
                    }
                }

                match std::fs::read_dir(path) {
                    Ok(entries) => {
                        let mut dirs = Vec::new();
                        let mut files = Vec::new();

                        for entry in entries.flatten() {
                            let file_name = entry.file_name().to_string_lossy().to_string();
                            let file_type = entry.file_type();
                            let is_dir = file_type.as_ref().map(|t| t.is_dir()).unwrap_or(false);

                            if is_long {
                                let meta = entry.metadata();
                                let size_bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                                if is_dir {
                                    dirs.push(format!("d  {file_name}/"));
                                } else {
                                    let size_str = if size_bytes < 1024 {
                                        format!("{size_bytes} B")
                                    } else if size_bytes < 1024 * 1024 {
                                        format!("{:.1} KB", size_bytes as f64 / 1024.0)
                                    } else {
                                        format!("{:.1} MB", size_bytes as f64 / (1024.0 * 1024.0))
                                    };
                                    files.push(format!("f  {file_name} ({size_str})"));
                                }
                            } else {
                                if is_dir {
                                    dirs.push(format!("{file_name}/"));
                                } else {
                                    files.push(file_name);
                                }
                            }
                        }

                        dirs.sort();
                        files.sort();

                        let mut all = dirs;
                        all.extend(files);

                        if all.is_empty() {
                            "(empty directory)".to_string()
                        } else {
                            all.join("\n")
                        }
                    }
                    Err(err) => format!("ls error: {err}"),
                }
            } else {
                let output = if cfg!(target_os = "windows") {
                    tokio::process::Command::new("powershell")
                        .args(["-NoProfile", "-Command", cmd_str])
                        .output()
                        .await
                } else {
                    tokio::process::Command::new("sh")
                        .args(["-c", cmd_str])
                        .output()
                        .await
                };

                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        let mut result = String::new();
                        if !stdout.is_empty() {
                            result.push_str(&stdout);
                        }
                        if !stderr.is_empty() {
                            if !result.is_empty() {
                                result.push('\n');
                            }
                            result.push_str(&stderr);
                        }
                        let cleaned = result
                            .lines()
                            .map(|l| l.trim_end())
                            .collect::<Vec<_>>()
                            .join("\n");
                        if cleaned.trim().is_empty() {
                            format!(
                                "(Command completed with code: {})",
                                out.status.code().unwrap_or(0)
                            )
                        } else {
                            cleaned.trim().to_string()
                        }
                    }
                    Err(err) => format!("Execution error: {err}"),
                }
            };

            send_reply(
                client,
                chat_jid,
                response,
                stanza_id,
                Some(sender_jid.to_string()),
                raw_message.clone(),
            )
            .await?;

            return Ok(());
        }

        let matched_prefix = config
            .prefixes
            .iter()
            .find(|p| trimmed.starts_with(p.as_str()));

        let prefix = match matched_prefix {
            Some(p) => p,
            None => return Ok(()),
        };

        if !config.has_permission(&sender_jid.to_string(), is_from_me) {
            return Ok(());
        }

        let content = &trimmed[prefix.len()..];
        let mut tokens = content.split_whitespace();
        let command = tokens.next().unwrap_or("").to_lowercase();

        if command.is_empty() {
            return Ok(());
        }

        let ctx = PluginContext {
            client: client.clone(),
            chat_jid: chat_jid.clone(),
            sender_jid: sender_jid.clone(),
            stanza_id,
            raw_message: raw_message.clone(),
            config: config.clone(),
            start_time,
        };

        if command == "help" || command == "menu" {
            let mut list = format!(
                "Bot Menu\nMode: {}\nPrefixes: {}\n\nPerintah:\n",
                config.mode,
                config.prefixes.join(" ")
            );
            for p in plugin_manager.list() {
                list.push_str(&format!(
                    "- {} ({}) : {}\n",
                    p.name(),
                    p.commands().join(", "),
                    p.description()
                ));
            }
            list.push_str("- $ <cmd> : Shell exec (owner only)");
            send_text(client, chat_jid, list).await?;
            return Ok(());
        }

        if let Some(plugin) = plugin_manager.find(&command) {
            if let Err(err) = plugin.run(&ctx).await {
                eprintln!("Plugin error: {err}");
            }
        }

        Ok(())
    }
}
