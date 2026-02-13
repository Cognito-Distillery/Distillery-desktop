<p align="center">
  <img src="src-tauri/icons/128x128.png" width="96" alt="Distillery" />
</p>

<h1 align="center">Distillery</h1>

<p align="center">
  <em>Distill thoughts into aged knowledge.</em>
</p>

<p align="center">
  <strong><a href="docs/README_ko.md">한국어</a></strong>
</p>

---

```
  Malting       Malt House        Still          Queue           Cask
  ──────       ───────────      ──────       ──────────      ──────
 [ Idea ] ──→ [  Store  ] ──→ [ Select ] ──→ [ Distill ] ──→ [ Know ]
               ↑                   │
               └──── Draw Back ────┘
```

Distillery is a desktop app that **refines raw thoughts into structured knowledge**,
borrowing the process of a whiskey distillery as its metaphor.

Capture scattered decisions, problems, insights, and questions as **malts**.
Place them on the **still** to curate.
The server automatically distills them at noon and midnight,
aging them in the **cask** — your knowledge graph.

---

## The Process

### 1. Malting

Turn a thought into a malt. Choose one of four types:

| Type | Description |
|------|-------------|
| **Decision** | A finalized decision |
| **Problem** | An issue that needs resolution |
| **Insight** | A discovered insight |
| **Question** | A question that needs discussion |

### 2. Malt House

Your warehouse of malts. Search, edit, or delete them.

### 3. Still

Curate which malts to distill. When ready, send them to the distillation queue.

### 4. Draw Back

Pull malts out of the queue. Once they enter the cask, there is no turning back.

---

## Floating Memo (Quick Malt)

A small floating window for capturing thoughts instantly without switching to the main app.
The app stays in the **system tray** — close the main window and it keeps running in the background.

| Platform | Shortcut | Setup |
|----------|----------|-------|
| **macOS** | `Cmd+Shift+M` | Works out of the box |
| **Windows** | `Ctrl+Shift+M` | Works out of the box |
| **Linux (X11)** | `Ctrl+Shift+M` | Works out of the box |
| **Linux (Wayland)** | Any key you choose | Bind a DBus command in your compositor (see below) |

### Linux Wayland Setup

Wayland does not support app-level global shortcuts.
The app exposes a DBus service instead — bind the following command in your compositor/DE settings:

```bash
dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo
```

<details>
<summary>niri</summary>

Add to your `config.kdl` inside the `binds` block:

```kdl
Ctrl+Shift+M { spawn "dbus-send" "--session" "--type=method_call" "--dest=com.distillery.App" "/com/distillery/App" "com.distillery.App.ToggleFloatingMemo"; }
```

Then reload: `niri msg action reload-config`
</details>

<details>
<summary>Hyprland</summary>

Add to `hyprland.conf`:

```
bind = CTRL SHIFT, M, exec, dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo
```
</details>

<details>
<summary>GNOME</summary>

```bash
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/ name 'Distillery Floating Memo'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/ command "dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/ binding '<Ctrl><Shift>m'
```
</details>

<details>
<summary>KDE Plasma</summary>

System Settings > Shortcuts > Custom Shortcuts > Add new shortcut with:

- Trigger: `Ctrl+Shift+M`
- Command: `dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo`
</details>

---

## Tech Stack

```
Frontend    Svelte 5 · SvelteKit · TypeScript · Tailwind CSS · DaisyUI
Backend     Rust · SQLite (FTS5) · reqwest
Desktop     Tauri 2
```

---

## Getting Started

### Prerequisites

- [Bun](https://bun.sh) (or Node.js)
- [Rust toolchain](https://rustup.rs)
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)

### Install

```bash
bun install

cp src-tauri/.env.example src-tauri/.env
# Edit src-tauri/.env and set API_BASE_URL, WEB_BASE_URL
```

### Develop

```bash
bun run tauri dev
```

### Build

```bash
bun run tauri build
```

### Release (for forks)

1. Generate a signing key:

```bash
bun tauri signer generate -w ~/.tauri/distillery.key
```

2. Add the following **Repository Secrets** in GitHub (Settings → Secrets and variables → Actions):

| Secret | Value |
|--------|-------|
| `API_BASE_URL` | Your API server URL |
| `WEB_BASE_URL` | Your web app URL |
| `TAURI_SIGNING_PRIVATE_KEY` | Contents of the generated `.key` file |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Password used during key generation |

3. Push a version tag to trigger the build:

```bash
git tag v0.1.0
git push origin v0.1.0
```

A draft release will be created on GitHub with builds for Linux, macOS, and Windows.

---

## Project Structure

```
src/
├── routes/              # Pages (malting, malt house, still, drawback, settings, help)
├── lib/
│   ├── components/      # Svelte components
│   ├── stores/          # State management (auth, malts, settings)
│   ├── i18n/            # Internationalization (ko, en)
│   └── types.ts         # Type definitions
src-tauri/
├── src/
│   ├── commands.rs      # Tauri IPC commands
│   ├── db.rs            # SQLite database
│   ├── auth.rs          # OTP authentication
│   ├── api.rs           # Server API communication
│   └── models.rs        # Data models
```

---

<p align="center"><sub>pitch black, amber-lit.</sub></p>
