use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, Server, register_plugin,
    scheduler::SchedulerExt,
    text::TextComponent,
};
use tracing::info;

struct PingDisplayPlugin;

impl Plugin for PingDisplayPlugin {
    fn new() -> Self {
        Self
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "ping-display".into(),
            version: "0.1.2".into(),
            authors: vec!["you".into()],
            description: "Shows player ping (ms) next to their name in the tab list, color-coded by quality.".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Ping Display plugin loaded");

        // Update every 20 ticks (≈ 1 second)
        context.schedule_repeating_task(0, 20, |server: Server| {
            for player in server.get_all_players() {
                let name = player.get_name();
                let ping = player.get_ping();

                // §a green | §e yellow | §6 gold | §c red
                let colored = match ping {
                    0..=50 => format!("{} §a[{}ms]", name, ping),
                    51..=100 => format!("{} §e[{}ms]", name, ping),
                    101..=200 => format!("{} §6[{}ms]", name, ping),
                    _ => format!("{} §c[{}ms]", name, ping),
                };

                player.set_tab_list_name(Some(TextComponent::text(&colored)));
            }
        });

        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Ping Display plugin unloaded");
        Ok(())
    }
}

register_plugin!(PingDisplayPlugin);
