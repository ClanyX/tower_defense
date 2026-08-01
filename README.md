# 🛡️ Tower Defense Game

A high-performance, real-time 2D Tower Defense game built in **Rust** using the **Bevy** engine (v0.19). Defend your base against relentless waves of enemies, unlock powerful player upgrades, trigger abilities, and survive an adaptive score-based scaling system.

---

## 🌟 Key Features

- **🎮 Real-Time Arcade Tower Defense**: Defend your tower against continuous enemy waves.
- **⚡ Automatic Player Shooting**: Smart targeting system that automatically fires at the closest enemies when reloaded.
- **🎯 Multishot Ability**: Upgrade to attack two targets simultaneously (primary target takes 100% damage, secondary target takes 50% damage).
- **📈 Adaptive Score-Based Scaling**:
  - **Faster Wave Spawns**: Enemy spawn intervals accelerate from 2.0s down to 0.4s as your kill count grows.
  - **Scaling Enemies**: Enemy HP, movement speed, coin rewards, and tower damage scale dynamically with your score.
  - **Dynamic Upgrade Costs**: Upgrade prices start low for easy early-game progression and scale quadratically into the late game.
- **✨ Active Abilities**: Integrated bottom-left action bar for quick ability access.
- **⚙️ Launcher & In-Game Settings**:
  - Fullscreen toggle & resolution selection (720p HD, 1080p Full HD, 1440p QHD).
  - Pausable gameplay with mutually exclusive UI modals (Settings and Upgrade menu).

---

## 🕹️ Controls & Keybindings

| Key / Action | Function |
| :--- | :--- |
| `U` | Open / Close Upgrade Menu |
| `Escape` | Open / Close Settings Menu (or close current overlay) |
| `Mouse Click` | Interact with UI buttons (Upgrades, Settings, Launcher) |

---

## 🛠️ Technology Stack

- **Language**: [Rust](https://www.rust-lang.org/) (2024 Edition)
- **Game Engine**: [Bevy 0.19](https://bevyengine.org/) (Data-driven ECS Architecture)
- **Randomization**: `rand` crate

---

## 🚀 Getting Started

### Prerequisites

Ensure you have Rust and `cargo` installed on your machine. If not, install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation & Running

1. **Clone the repository**:
   ```bash
   git clone https://github.com/YourUsername/tower_defense.git
   cd tower_defense
   ```

2. **Run in Development Mode**:
   ```bash
   cargo run
   ```

3. **Build Release Version**:
   ```bash
   cargo build --release
   ```
   The compiled executable will be available under `target/release/`.

---

## 📂 Project Architecture

```
tower_defense/
├── Cargo.toml               # Project metadata & dependencies (Bevy 0.19)
└── src/
    ├── main.rs              # App entry point, state management, & system schedules
    └── app/
        ├── mod.rs
        ├── game/            # Gameplay systems & entities
        │   ├── mod.rs
        │   ├── logic.rs     # Main loop, spawning, shooting, collision & scoring
        │   ├── enemy.rs     # Enemy struct, movement, HP display & damage logic
        │   ├── player.rs    # Player entity, stats (Damage, Reload, Multishot)
        │   └── tower.rs     # Tower entity & health state
        └── ui/              # User Interface & Modals
            ├── mod.rs
            ├── settings.rs  # Launcher menu, settings overlay & display modes
            ├── upgrade.rs   # Upgrade shop modal & score-scaled pricing logic
            └── ability.rs   # In-game bottom-left ability action bar
```

---

## 📜 License

This project is open-source and available under the [MIT License](LICENSE).
