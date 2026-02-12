# Distillery

> *Distill thoughts into aged knowledge.*

**[한국어](docs/README_ko.md)**

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
# Install dependencies
bun install

# Set up environment variables
cp src-tauri/.env.example src-tauri/.env
# Edit src-tauri/.env and set API_BASE_URL
```

### Develop

```bash
# Start Tauri dev server
bun run tauri dev
```

### Build

```bash
# Production build
bun run tauri build
```

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

<sub>pitch black, amber-lit.</sub>
