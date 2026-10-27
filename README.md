# rsgit

A terminal user interface (TUI) Git client written in Rust.

## Features

- Browse local branches, remote branches, and tags
- Checkout, create, rename, and delete refs
- Merge and rebase with selectable strategies
- Pull and push with inline progress
- Inspect commit history and commit diffs
- Keyboard-driven interface with contextual help

## Requirements

- [Git](https://git-scm.com/)
- Rust 1.85 or newer (edition 2024)

## Build and run

```sh
cargo run --release
```

To open a specific repository:

```sh
cargo run --release -- /path/to/repository
```

## Key bindings

| Key | Action |
| --- | --- |
| `1`-`5` | Jump to pane |
| `Tab` / `h` | Next pane |
| `Shift-Tab` / `l` | Previous pane |
| `j` / `Down` | Move selection down |
| `k` / `Up` | Move selection up |
| `[` / `]` | Switch branch tabs |
| `Enter` | Checkout selected ref / confirm popup |
| `n` | Create branch or tag |
| `r` | Rename selected item |
| `d` | Delete selected item |
| `p` / `P` | Pull / push selected branch |
| `M` | Merge current branch into selected branch |
| `R` | Rebase current branch onto selected branch |
| `?` | Toggle help |
| `a` | Toggle about |
| `q` / `Esc` | Close popup / quit |

## Development

Before committing, run:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## License

MIT
