use ratatui::style::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    // Enemies
    LostShade,    // weak, confused
    Lampad,       // underworld nymph, moderate
    Eurynomos,    // flesh-eating daemon
    Empusa,       // shapeshifter, dangerous
    // Items
    Nectar,       // heals
    Obol,         // score/currency
    Moly,         // temporary strength boost
    StygianBlade, // weapon upgrade
}

impl EntityKind {
    pub fn glyph(self) -> char {
        match self {
            EntityKind::Player => '@',
            EntityKind::LostShade => 's',
            EntityKind::Lampad => 'l',
            EntityKind::Eurynomos => 'E',
            EntityKind::Empusa => 'M',
            EntityKind::Nectar => '!',
            EntityKind::Obol => '$',
            EntityKind::Moly => '?',
            EntityKind::StygianBlade => '/',
        }
    }

    pub fn color(self) -> Color {
        match self {
            EntityKind::Player => Color::Yellow,
            EntityKind::LostShade => Color::DarkGray,
            EntityKind::Lampad => Color::Magenta,
            EntityKind::Eurynomos => Color::Red,
            EntityKind::Empusa => Color::LightRed,
            EntityKind::Nectar => Color::Green,
            EntityKind::Obol => Color::Yellow,
            EntityKind::Moly => Color::Cyan,
            EntityKind::StygianBlade => Color::White,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            EntityKind::Player => "You",
            EntityKind::LostShade => "Lost Shade",
            EntityKind::Lampad => "Lampad",
            EntityKind::Eurynomos => "Eurynomos",
            EntityKind::Empusa => "Empusa",
            EntityKind::Nectar => "Nectar",
            EntityKind::Obol => "Obol",
            EntityKind::Moly => "Moly",
            EntityKind::StygianBlade => "Stygian Blade",
        }
    }

    pub fn is_enemy(self) -> bool {
        matches!(self, EntityKind::LostShade | EntityKind::Lampad
            | EntityKind::Eurynomos | EntityKind::Empusa)
    }

    pub fn is_item(self) -> bool {
        matches!(self, EntityKind::Nectar | EntityKind::Obol
            | EntityKind::Moly | EntityKind::StygianBlade)
    }
}

pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub kind: EntityKind,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub alive: bool,
    pub strength_turns: i32, // moly buff remaining turns
}

impl Entity {
    pub fn player(x: i32, y: i32) -> Self {
        Entity {
            x, y,
            kind: EntityKind::Player,
            hp: 40, max_hp: 40,
            attack: 5, defense: 3,
            alive: true,
            strength_turns: 0,
        }
    }

    pub fn enemy(x: i32, y: i32, kind: EntityKind, depth: u32) -> Self {
        let scale = 1.0 + (depth as f32 - 1.0) * 0.15;
        let (hp, atk, def) = match kind {
            EntityKind::LostShade => (
                (8.0 * scale) as i32,
                (2.0 * scale) as i32,
                0,
            ),
            EntityKind::Lampad => (
                (14.0 * scale) as i32,
                (4.0 * scale) as i32,
                (1.0 * scale) as i32,
            ),
            EntityKind::Eurynomos => (
                (20.0 * scale) as i32,
                (6.0 * scale) as i32,
                (2.0 * scale) as i32,
            ),
            EntityKind::Empusa => (
                (25.0 * scale) as i32,
                (8.0 * scale) as i32,
                (3.0 * scale) as i32,
            ),
            _ => (1, 0, 0),
        };
        Entity {
            x, y, kind,
            hp, max_hp: hp,
            attack: atk, defense: def,
            alive: true,
            strength_turns: 0,
        }
    }

    pub fn item(x: i32, y: i32, kind: EntityKind) -> Self {
        Entity {
            x, y, kind,
            hp: 0, max_hp: 0,
            attack: 0, defense: 0,
            alive: true,
            strength_turns: 0,
        }
    }

    pub fn effective_attack(&self) -> i32 {
        if self.strength_turns > 0 {
            self.attack + 4
        } else {
            self.attack
        }
    }
}
