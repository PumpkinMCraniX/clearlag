# ClearLag – PumpkinMC Plugin

Clears ground entities on a timer.

## Default clear list (`clear_other_entities = false`)

- Dropped items (`Item`)
- Experience orbs (`ExperienceOrb`)
- Arrows & tipped arrows (`Arrow`)
- Spectral arrows (`SpectralArrow`)
- Snowballs (`Snowball`)
- Eggs (`Egg`)

Players and mobs are **not** removed in this mode.

## Aggressive mode (`clear_other_entities = true`)

Removes **everything that is not a player** (including mobs).

## Config (`plugins/data/ClearLag/config.toml`)

```toml
interval_minutes = 10
clear_other_entities = false
clear_message = "§a[ClearLag] Removed {count} ground entities."
```

## Command

| Command     | Permission    | Description              |
|-------------|---------------|--------------------------|
| `/clearlag` | `clearlag.use` | Clear entities immediately |

## Build

```bash
rustup target add wasm32-wasip2
cargo build --release
```

Pin `pumpkin-plugin-api` in `Cargo.toml` to the **same** Pumpkin tag/commit as your server binary, or you will get API version mismatch errors at load time.
