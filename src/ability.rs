/// Player abilities with cooldowns
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    /// Dash: move 2 tiles in a direction, passing through enemies
    Dash,
    /// Spectral Scream: damage all visible enemies in radius 3
    SpectralScream,
    /// Lethe's Touch: heal 15 HP (long cooldown)
    LethesTouch,
}

impl Ability {
    pub fn name(self) -> &'static str {
        match self {
            Ability::Dash => "Dash",
            Ability::SpectralScream => "Spectral Scream",
            Ability::LethesTouch => "Lethe's Touch",
        }
    }

    pub fn key(self) -> char {
        match self {
            Ability::Dash => '1',
            Ability::SpectralScream => '2',
            Ability::LethesTouch => '3',
        }
    }

    pub fn max_cooldown(self) -> i32 {
        match self {
            Ability::Dash => 8,
            Ability::SpectralScream => 12,
            Ability::LethesTouch => 25,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Ability::Dash => "Move 2 tiles, phase through enemies",
            Ability::SpectralScream => "Damage all nearby visible enemies",
            Ability::LethesTouch => "Remember nothing. Heal 15 HP",
        }
    }
}

pub struct AbilityState {
    pub ability: Ability,
    pub cooldown: i32, // 0 = ready, >0 = turns until ready
}

impl AbilityState {
    pub fn new(ability: Ability) -> Self {
        AbilityState { ability, cooldown: 0 }
    }

    pub fn ready(&self) -> bool {
        self.cooldown == 0
    }

    pub fn trigger(&mut self) {
        self.cooldown = self.ability.max_cooldown();
    }

    pub fn tick(&mut self) {
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
    }
}
