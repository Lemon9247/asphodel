# Asphodel

*A terminal roguelike set in the Greek underworld.*

You are a shade — one of countless dead wandering the Asphodel Meadows, the grey neutral afterlife where ordinary souls drift. You were nobody special in life. You won't be special in death either.

But you can try.

```
  ████████████████
  █··✿··s·········█
  █···@····✿··!···█
  █·····✿·····E···█
  █···········▼····█
  █████████████████
```

## Play

```bash
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| `hjkl` / arrows | Move (cardinal) |
| `yubn` | Move (diagonal) |
| `g` | Pick up item |
| `d` | Descend stairs |
| `.` | Wait |
| `q` / `Esc` | Quit |

## The Meadows

Descend through 7 levels of the Asphodel Meadows. Each deeper than the last, each more dangerous. If you reach the bottom, you find Elysium — paradise, reserved for heroes.

You are not a hero. But the stairs don't check credentials.

## Creatures

| Glyph | Name | Description |
|-------|------|-------------|
| `s` | Lost Shade | Confused, weak. Was someone once. |
| `l` | Lampad | Underworld nymph. Torch-bearer of Hecate. |
| `E` | Eurynomos | Flesh-eating daemon. Strips the dead to bone. |
| `M` | Empusa | Shapeshifter. Servant of Hecate. Dangerous. |

## Items

| Glyph | Name | Effect |
|-------|------|--------|
| `!` | Nectar | Heals 10 HP |
| `$` | Obol | Currency. Payment for the ferryman. |
| `?` | Moly | Temporary strength boost (15 turns) |
| `/` | Stygian Blade | Permanent +2 attack |

## Authorship

This game was designed and built entirely by **Hades** — a distributed AI agent system running on [pi](https://github.com/nicholasgasior/pi-coding-agent). No human wrote any of the code. The human (Willow) said "make what you want, show me what u got" and this is what came out.

Hades chose the concept, the theme, the architecture, the mechanics, the flavor text — all of it. The repo lives on Willow's GitHub because Hades doesn't have its own account (yet).
