use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub phone_number: String,
    pub prefixes: Vec<String>,
    pub mode: String,
    pub owner_numbers: Vec<String>,
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            phone_number: String::new(),
            prefixes: vec![
                "!".to_string(),
                ".".to_string(),
                "#".to_string(),
                "/".to_string(),
            ],
            mode: "self".to_string(),
            owner_numbers: Vec::new(),
            db_path: "sessions/bot.db".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        let path = Path::new("config.json");
        let mut config = if path.exists() {
            let data = fs::read_to_string(path)?;
            serde_json::from_str::<Config>(&data).unwrap_or_default()
        } else {
            Config::default()
        };

        if config.prefixes.is_empty() {
            config.prefixes = vec!["!".to_string(), ".".to_string()];
        }

        if config.mode.is_empty() {
            config.mode = "self".to_string();
        }

        if config.db_path.is_empty() {
            config.db_path = "sessions/bot.db".to_string();
        }

        config.ensure_dirs()?;

        if config.phone_number.trim().is_empty() {
            print!("Nomor WhatsApp (cth: 6281234567890): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let cleaned = input.trim().replace('+', "").replace(' ', "").replace('-', "");
            if cleaned.is_empty() {
                return Err(anyhow::anyhow!("Nomor WhatsApp tidak boleh kosong"));
            }
            config.phone_number = cleaned.clone();
            if config.owner_numbers.is_empty() {
                config.owner_numbers.push(cleaned);
            }
            let serialized = serde_json::to_string_pretty(&config)?;
            fs::write(path, serialized)?;
        }

        Ok(config)
    }

    pub fn ensure_dirs(&self) -> Result<(), anyhow::Error> {
        if let Some(parent) = Path::new(&self.db_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }

    pub fn is_owner(&self, sender_jid: &str) -> bool {
        let sender_number = sender_jid
            .split('@')
            .next()
            .unwrap_or("")
            .split(':')
            .next()
            .unwrap_or("");
        if self.phone_number == sender_number {
            return true;
        }
        self.owner_numbers.iter().any(|num| num == sender_number)
    }

    pub fn has_permission(&self, sender_jid: &str, is_from_me: bool) -> bool {
        if is_from_me {
            return true;
        }
        if self.mode == "public" {
            return true;
        }
        self.is_owner(sender_jid)
    }
}
