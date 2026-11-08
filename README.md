# tuikit

`tuikit` is an internal shared collection of **generic, reusable primitives** for terminal UI applications built with [Ratatui](https://ratatui.rs/) and [Crossterm](https://github.com/crossterm-rs/crossterm).

It extracts app-agnostic infrastructure commonly reimplemented across TUIs: terminal session lifecycle, HTTP request helpers, XDG-ish path/config utilities, theme/color math, connectivity verdicts, and optional palette/effects/cache helpers.

## Design goals

- **Generic names and APIs** — no application/domain coupling.
- **Feature-gated dependencies** — pay only for what you use.
- **Internal convention** — shared across craftty-org TUI projects via git dependency.
- **Practical primitives** — small building blocks, not a framework.

## Install

```toml
[dependencies]
tuikit = { git = "https://github.com/craftty-org/tuikit", features = ["terminal", "http", "config", "theme-catalog"] }
```

## Features

| Feature | Purpose |
|---|---|
| `terminal` | `TerminalSession` guard (raw mode, alternate screen, mouse/paste, panic restore) |
| `http` | Generic HTTP helpers (retry, JSON with size limit, streaming download, errors) and connectivity verdict |
| `config` | TOML load/save helpers and `dirs`-based app path helpers |
| `fslog` | Simple append log file + Unix stderr redirect helper |
| `theme-catalog` | Schema-light theme catalog entries/load/merge (`name`, `light`, color map) |
| `palette` | Palette extraction from image bytes (`image` + `color-thief`) |
| `effects` | Thin optional `tachyonfx` wrapper |
| `cache-sqlite` | Optional SQLite open/WAL helpers |
| `cache-codec` | Optional zstd + serde blob codec helpers |
| `full` | Enables all features |

## Modules

Always available (no heavy deps): `time`, `text`, `theme::color`, `theme::contrast`, `theme::hsv`.

Feature-gated: `terminal`, `http`, `paths`, `config`, `fslog`, `theme::catalog`, `theme::palette`, `effects`, `cache`.

## License

GPL-3.0-only. See [`LICENSE`](./LICENSE).
