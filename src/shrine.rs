use rand::Rng;

/// Shrine boons — the player picks one when they interact
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Boon {
    /// +10 max HP and heal to full
    VitalityOfErebus,
    /// +3 attack
    WrathOfAres,
    /// +2 defense
    AegisOfAthena,
    /// Reduce all ability cooldowns by 3
    SwiftnessOfHermes,
    /// +15 HP heal
    GraceOfPersephone,
    /// See further (FOV +3)
    EyesOfNyx,
}

impl Boon {
    pub fn name(self) -> &'static str {
        match self {
            Boon::VitalityOfErebus => "Vitality of Erebus",
            Boon::WrathOfAres => "Wrath of Ares",
            Boon::AegisOfAthena => "Aegis of Athena",
            Boon::SwiftnessOfHermes => "Swiftness of Hermes",
            Boon::GraceOfPersephone => "Grace of Persephone",
            Boon::EyesOfNyx => "Eyes of Nyx",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Boon::VitalityOfErebus => "+10 max HP, heal to full",
            Boon::WrathOfAres => "+3 attack",
            Boon::AegisOfAthena => "+2 defense",
            Boon::SwiftnessOfHermes => "All cooldowns reduced by 3",
            Boon::GraceOfPersephone => "Heal 15 HP",
            Boon::EyesOfNyx => "See further in the dark",
        }
    }

    pub fn random_pair() -> (Boon, Boon) {
        let all = [
            Boon::VitalityOfErebus,
            Boon::WrathOfAres,
            Boon::AegisOfAthena,
            Boon::SwiftnessOfHermes,
            Boon::GraceOfPersephone,
            Boon::EyesOfNyx,
        ];
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0..all.len());
        let mut b = rng.gen_range(0..all.len() - 1);
        if b >= a { b += 1; }
        (all[a], all[b])
    }
}

/// A shrine placed on the map
pub struct Shrine {
    pub x: i32,
    pub y: i32,
    pub boon_a: Boon,
    pub boon_b: Boon,
    pub used: bool,
}

impl Shrine {
    pub fn new(x: i32, y: i32) -> Self {
        let (a, b) = Boon::random_pair();
        Shrine {
            x, y,
            boon_a: a,
            boon_b: b,
            used: false,
        }
    }
}
