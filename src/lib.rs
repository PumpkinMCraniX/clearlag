//! ClearLag – PumpkinMC plugin
//!
//! Periodically removes ground entities (dropped items, XP orbs, arrows,
//! snowballs, eggs by default) and provides `/clearlag` (permission-restricted).
//!
//! Interval and behaviour are configurable via `config.toml` in the plugin data folder.

use std::path::PathBuf;
use std::sync::Mutex;

use pumpkin_plugin_api::{
    command::{Command, CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    logging::{self, LogLevel},
    permissions,
    register_plugin,
    scheduler::SchedulerExt,
    text::TextComponent,
    Context, EntityType, Plugin, PluginMetadata, Result, Server, World,
};
use serde::{Deserialize, Serialize};

const DEFAULT_INTERVAL_MINUTES: u64 = 10;
const TICKS_PER_SECOND: u64 = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    /// How often (in minutes) to automatically clear ground entities
    interval_minutes: u64,
    /// If true, remove everything that isn't a player (including mobs).
    /// If false, only items, XP orbs, arrows, spectral arrows, snowballs, and eggs.
    clear_other_entities: bool,
    /// Message shown when entities are removed. `{count}` is substituted.
    clear_message: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interval_minutes: DEFAULT_INTERVAL_MINUTES,
            clear_other_entities: false,
            clear_message: "§a[ClearLag] Removed {count} ground entities.".to_string(),
        }
    }
}

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

struct ClearLagPlugin;

impl Plugin for ClearLagPlugin {
    fn new() -> Self {
        ClearLagPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "ClearLag".into(),
            version: "0.1.3".into(),
            authors: vec!["YourName".into()],
            description: "Clears ground entities every N minutes or via /clearlag".into(),
            dependencies: vec![],
            permissions: vec![permissions::FS_WRITE_DATA.into()],
        }
    }

    fn on_load(&self, context: Context) -> Result<()> {
        logging::log(LogLevel::Info, "ClearLag loading...");

        let config = load_config(&context);
        let interval_minutes = config.interval_minutes.max(1);
        let period_ticks = interval_minutes * 60 * TICKS_PER_SECOND;

        {
            let mut guard = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
            *guard = Some(config);
        }

        let cmd = Command::new(
            &[String::from("clearlag")],
            "Clears ground entities to reduce lag.",
        )
        .execute(ClearLagCommand);
        context.register_command(cmd, "clearlag.use");

        context.schedule_repeating_task(period_ticks, period_ticks, |server: Server| {
            let (clear_other, message_template) = {
                let guard = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
                match guard.as_ref() {
                    Some(cfg) => (cfg.clear_other_entities, cfg.clear_message.clone()),
                    None => (false, Config::default().clear_message),
                }
            };

            let count = clear_ground_entities(&server, clear_other);
            if count > 0 {
                let _msg = message_template.replace("{count}", &count.to_string());
                logging::log(
                    LogLevel::Info,
                    &format!("ClearLag: removed {count} entities"),
                );
            }
        });

        logging::log(
            LogLevel::Info,
            &format!("ClearLag loaded – clearing every {interval_minutes} minute(s)"),
        );
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> Result<()> {
        logging::log(LogLevel::Info, "ClearLag unloaded");
        Ok(())
    }
}

struct ClearLagCommand;

impl CommandHandler for ClearLagCommand {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> std::result::Result<i32, CommandError> {
        let (clear_other, message_template) = {
            let guard = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
            match guard.as_ref() {
                Some(cfg) => (cfg.clear_other_entities, cfg.clear_message.clone()),
                None => (false, Config::default().clear_message),
            }
        };

        let count = clear_ground_entities(&server, clear_other);
        let msg = message_template.replace("{count}", &count.to_string());
        sender.send_message(TextComponent::text(&msg));
        Ok(0)
    }
}

fn load_config(ctx: &Context) -> Config {
    let data_folder = PathBuf::from(ctx.get_data_folder());
    let config_path = data_folder.join("config.toml");

    if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => match toml::from_str::<Config>(&content) {
                Ok(cfg) => return cfg,
                Err(e) => {
                    logging::log(
                        LogLevel::Warn,
                        &format!("Failed to parse config.toml: {e}. Using defaults."),
                    );
                }
            },
            Err(e) => {
                logging::log(
                    LogLevel::Warn,
                    &format!("Failed to read config.toml: {e}. Using defaults."),
                );
            }
        }
    }

    let default = Config::default();
    if let Ok(toml_str) = toml::to_string_pretty(&default) {
        let _ = std::fs::create_dir_all(&data_folder);
        if let Err(e) = std::fs::write(&config_path, &toml_str) {
            logging::log(
                LogLevel::Warn,
                &format!("Could not write default config.toml: {e}"),
            );
        } else {
            logging::log(LogLevel::Info, "Created default config.toml");
        }
    }
    default
}

fn clear_ground_entities(server: &Server, clear_other: bool) -> usize {
    let mut removed = 0;
    for world in server.get_all_worlds() {
        removed += clear_world_entities(&world, clear_other);
    }
    removed
}

fn clear_world_entities(world: &World, clear_other: bool) -> usize {
    let mut removed = 0;

    for entity in world.get_entities() {
        let entity_type = entity.get_type();

        let should_remove = if clear_other {
            // Everything except players (includes mobs)
            entity_type != EntityType::Player
        } else {
            // Safe lag clear: items, XP, arrows, spectral arrows, snowballs, eggs
            matches!(
                entity_type,
                EntityType::Item
                    | EntityType::ExperienceOrb
                    | EntityType::Arrow
                    | EntityType::SpectralArrow
                    | EntityType::Snowball
                    | EntityType::Egg
            )
        };

        if should_remove {
            entity.remove();
            removed += 1;
        }
    }

    removed
}

register_plugin!(ClearLagPlugin);
