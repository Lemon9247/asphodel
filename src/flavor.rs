use rand::Rng;

/// Ambient messages that play occasionally to build atmosphere
pub fn ambient_message(depth: u32, turns: u32) -> Option<&'static str> {
    let mut rng = rand::thread_rng();

    // Only trigger occasionally
    if !rng.gen_ratio(1, 25) {
        return None;
    }

    let pool: &[&str] = match depth {
        1 => &[
            "The grey flowers sway in a wind you can't feel.",
            "Somewhere far off, a shade is weeping.",
            "The air tastes of nothing. You remember nothing.",
            "Pale petals drift past. They have no scent.",
            "You hear footsteps. Yours? Someone else's? Does it matter?",
            "The ceiling is just... grey. There is no sky here.",
        ],
        2 => &[
            "The flowers here grow taller. They lean toward you.",
            "A whisper: a name you almost recognize.",
            "The walls weep with condensation. Or is it tears?",
            "You step on something soft. You don't look down.",
            "Was that always a corridor? You could have sworn...",
        ],
        3 => &[
            "The asphodels here have thorns. They didn't before.",
            "You hear the river. Distant. Always distant.",
            "A shade passes through the wall, unseeing.",
            "The darkness between rooms feels thicker now.",
            "Something scratches at the edge of your memory.",
        ],
        4 => &[
            "The flowers here are grey. Were they always grey?",
            "You feel yourself becoming lighter. Less.",
            "The walls pulse like a heartbeat. Slow. Ancient.",
            "You think you remember sunlight. You're probably wrong.",
            "A lampad's torch flickers in a room you've already left.",
        ],
        5 => &[
            "The meadows are narrowing. Pressing in.",
            "You hear your name. You don't remember your name.",
            "The asphodels have eyes. No. Just flowers. Just flowers.",
            "Something about these corridors feels deliberate.",
            "You are closer to the bottom than to the surface.",
        ],
        6 => &[
            "The air hums with something that isn't sound.",
            "You can almost see through the walls now.",
            "The dead are thicker here. They press against you.",
            "Below you: warmth. Above you: nothing you want.",
            "The flowers whisper. You choose not to listen.",
        ],
        7 => &[
            "Light. Ahead. Not torchlight. Something else.",
            "The asphodels part before you. Reluctantly.",
            "You feel the pull of Lethe. Sweet. Tempting.",
            "Almost there. Almost somewhere.",
            "The flowers thin. The walls retreat. Space opens.",
        ],
        _ => &[
            "The grey stretches on.",
        ],
    };

    Some(pool[rng.gen_range(0..pool.len())])
}

/// Messages on entering a new depth
pub fn depth_message(depth: u32) -> &'static str {
    match depth {
        1 => "The Asphodel Meadows. First terrace.",
        2 => "Deeper. The flowers grow wilder.",
        3 => "The middle meadows. The shades here are restless.",
        4 => "The grey deepens. The dead press closer.",
        5 => "The deep meadows. Few shades wander willingly here.",
        6 => "Near the bottom. The air trembles.",
        7 => "The final terrace. Beyond this: Elysium, or oblivion.",
        _ => "Deeper still.",
    }
}
