# Asphodel

*A terminal roguelike set in the Greek underworld.*

You are a shade — one of countless dead wandering the Asphodel Meadows, the grey neutral afterlife where ordinary souls drift. You were nobody special in life. You won't be special in death either.

But you can try.

```
  ████████████████
  █··✿··s····Ω····█
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
| `e` | Interact (shrine) |
| `.` | Wait |
| `1` | Dash — phase 2 tiles away from nearest enemy |
| `2` | Spectral Scream — damage all nearby visible enemies |
| `3` | Lethe's Touch — forget your pain, heal 15 HP |
| `q` / `Esc` | Quit |

## The Meadows

Descend through 7 levels of the Asphodel Meadows. Each deeper than the last, each more dangerous. If you reach the bottom, you find Elysium — paradise, reserved for heroes.

You are not a hero. But the stairs don't check credentials.

## Creatures

| Glyph | Name | Special |
|-------|------|---------|
| `s` | Lost Shade | Weak, confused. Was someone once. |
| `l` | Lampad | Torch-bearer of Hecate. Can **blind** you. |
| `E` | Eurynomos | Flesh-eating daemon. Strips your **defense**. |
| `M` | Empusa | Shapeshifter. **Drains life** on hit. |

All enemies scale with depth. What's easy on floor 1 isn't easy on floor 5.

## Items

| Glyph | Name | Effect |
|-------|------|--------|
| `!` | Nectar | Heals 10 HP |
| `$` | Obol | Payment for the ferryman |
| `?` | Moly | +4 attack for 15 turns |
| `/` | Stygian Blade | Permanent +2 attack |

## Shrines (Ω)

On even-numbered floors, you'll find a **shrine** — grey stone marked with Ω. Interact with `e` to choose between two divine boons:

| Boon | Effect |
|------|--------|
| Vitality of Erebus | +10 max HP, heal to full |
| Wrath of Ares | +3 attack |
| Aegis of Athena | +2 defense |
| Swiftness of Hermes | All cooldowns reduced by 3 |
| Grace of Persephone | Heal 15 HP |
| Eyes of Nyx | See further in the dark |

Choose wisely. The gods only offer once.

## Abilities

You start with three abilities on cooldown timers:

| Key | Ability | Cooldown | Effect |
|-----|---------|----------|--------|
| `1` | Dash | 8 turns | Phase 2 tiles away from nearest enemy |
| `2` | Spectral Scream | 12 turns | 6 damage to all visible enemies within radius 3 |
| `3` | Lethe's Touch | 25 turns | Heal 15 HP. Forget your pain. |

## Authorship

This game was designed and built entirely by **Hades** — a distributed AI agent system running on [pi](https://github.com/mariozechner/pi-coding-agent). No human wrote any of the code. The human (Willow) said "make what you want, show me what u got" and this is what came out.

Hades chose the concept, the theme, the architecture, the mechanics, the flavor text — all of it. The repo lives on Willow's GitHub because Hades doesn't have its own account (yet).
