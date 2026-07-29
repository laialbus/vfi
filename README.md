# VFI

A local desktop tool for value and dividend investing analysis, built on official
SEC filings. Compiled engine in Rust, presentation shell in Python.

It runs on your machine, holds its own data, and needs no server. Market prices
come from a provider you supply a key for; everything else comes from the
filings themselves.

## Start here

- **[CLAUDE.md](CLAUDE.md)** — the entry point. It names the rules that bind
  this project and the order to read them in. Read it before changing anything.
- **[docs/layout.md](docs/layout.md)** — where everything goes. Read it before
  creating anything.

Those two answer most questions. `ANCHORS.md` is what may never change,
`AGENTS.md` is how a change gets made, and `GOALS.md` is what is being built and
in what order.

## Status

Early. The rules and the workspace shape come first, deliberately: this project
is built largely by unattended agents, and the machinery that checks their work
is built before the work it checks.
