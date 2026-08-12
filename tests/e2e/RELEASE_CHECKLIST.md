# Factor release checklist (manual)

## Privacy / strip

- [ ] Fresh install: no login wall
- [ ] Firewall deny-all: shell works
- [ ] No unexpected prompts for other apps’ data (macOS)
- [ ] About identifies Factor; AGPL/MIT notices present

## Agent / BYOP

- [ ] Add Ollama (or OpenAI-compat) provider
- [ ] Agent: “what is cwd and list top-level files” succeeds with tools
- [ ] Multi-turn: agent runs command, reads result, answers
- [ ] Second provider type works (if configured)

## Workflow

- [ ] Bash → agent → bash preserves session
- [ ] Failed command can be sent to agent with context
- [ ] CLI agent (`claude`/`codex` if installed) gets status treatment

## Orchestration

- [ ] Local child agent spawn works offline (no warp.dev)
- [ ] No cloud Oz / capacity UI

## SSH (if available)

- [ ] Warpify SSH session
- [ ] Agent shell tool runs remotely

## Packaging

- [ ] macOS app launches from Finder
- [ ] Linux binary/AppImage launches
- [ ] Logs written to Factor paths only
