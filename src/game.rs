use rand::Rng;
use crate::ability::{Ability, AbilityState};
use crate::entity::{Entity, EntityKind};
use crate::fov::compute_fov;
use crate::map::Map;
use crate::shrine::{Boon, Shrine};
use crate::spawn::populate_room;

const LOG_SIZE: usize = 8;

pub enum GameState {
    Title,
    Playing,
    Dead,
    Victory,
}

pub enum Action {
    Move(i32, i32),
    Wait,
    Pickup,
    Descend,
    Interact,
    UseAbility(usize),
}

pub struct Game {
    pub map: Map,
    pub player: Entity,
    pub entities: Vec<Entity>,
    pub shrines: Vec<Shrine>,
    pub log: Vec<String>,
    pub depth: u32,
    pub obols: u32,
    pub turns: u32,
    pub game_over: bool,
    pub victory: bool,
    pub killed_by: String,
    pub abilities: Vec<AbilityState>,
    pub fov_radius: i32,
    pub shrine_prompt: Option<usize>, // index into shrines vec
    pub blind_turns: i32, // blinded by lampad
}

impl Game {
    pub fn new() -> Self {
        let depth = 1;
        let map = Map::new(depth);
        let (px, py) = map.rooms[0].center();
        let player = Entity::player(px as i32, py as i32);

        let mut entities = Vec::new();
        let mut shrines = Vec::new();
        for i in 1..map.rooms.len() {
            let room = map.rooms[i];
            populate_room(&room, depth, &mut entities, &map);
        }
        // Place shrine in a random middle room (not first or last)
        if map.rooms.len() > 3 {
            let shrine_room = &map.rooms[map.rooms.len() / 2];
            let (sx, sy) = shrine_room.center();
            shrines.push(Shrine::new(sx as i32, sy as i32));
        }

        let abilities = vec![
            AbilityState::new(Ability::Dash),
            AbilityState::new(Ability::SpectralScream),
            AbilityState::new(Ability::LethesTouch),
        ];

        let mut game = Game {
            map, player, entities, shrines,
            log: Vec::new(),
            depth, obols: 0, turns: 0,
            game_over: false, victory: false,
            killed_by: String::new(),
            abilities,
            fov_radius: 8,
            shrine_prompt: None,
            blind_turns: 0,
        };
        compute_fov(&mut game.map, game.player.x, game.player.y, game.fov_radius);
        game
    }

    pub fn log(&mut self, msg: &str) {
        self.log.push(msg.to_string());
        if self.log.len() > LOG_SIZE {
            self.log.remove(0);
        }
    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }

    pub fn effective_fov(&self) -> i32 {
        if self.blind_turns > 0 { 3 } else { self.fov_radius }
    }

    pub fn player_action(&mut self, action: Action) {
        if self.game_over {
            return;
        }

        match action {
            Action::Move(dx, dy) => self.try_move(dx, dy),
            Action::Wait => {
                self.log("You wait. The meadows don't care.");
            }
            Action::Pickup => self.try_pickup(),
            Action::Descend => self.try_descend(),
            Action::Interact => self.try_interact(),
            Action::UseAbility(idx) => self.try_ability(idx),
        }

        // Tick buffs
        if self.player.strength_turns > 0 {
            self.player.strength_turns -= 1;
            if self.player.strength_turns == 0 {
                self.log("The moly's power fades.");
            }
        }
        if self.blind_turns > 0 {
            self.blind_turns -= 1;
            if self.blind_turns == 0 {
                self.log("Your vision clears.");
            }
        }

        // Tick ability cooldowns
        for ab in &mut self.abilities {
            ab.tick();
        }

        // Enemy turns
        if !self.game_over {
            self.enemy_turns();
        }
        self.turns += 1;

        // Recompute FOV
        let fov = self.effective_fov();
        compute_fov(&mut self.map, self.player.x, self.player.y, fov);
    }

    fn try_move(&mut self, dx: i32, dy: i32) {
        let nx = self.player.x + dx;
        let ny = self.player.y + dy;

        if !self.map.in_bounds(nx, ny) {
            return;
        }
        if !self.map.tiles[ny as usize][nx as usize].walkable() {
            return;
        }

        // Check for enemy at target
        if let Some(idx) = self.entities.iter().position(|e| e.x == nx && e.y == ny && e.alive && e.kind.is_enemy()) {
            self.attack_entity(idx);
            return;
        }

        self.player.x = nx;
        self.player.y = ny;
    }

    fn attack_entity(&mut self, idx: usize) {
        let mut rng = rand::thread_rng();
        let atk = self.player.effective_attack();
        let def = self.entities[idx].defense;
        let damage = (atk - def + rng.gen_range(-1..=2)).max(0);

        self.entities[idx].hp -= damage;
        let name = self.entities[idx].kind.name();

        if damage > 0 {
            self.log(&format!("You strike the {} for {} damage.", name, damage));
        } else {
            self.log(&format!("You strike the {} but deal no damage.", name));
        }

        if self.entities[idx].hp <= 0 {
            self.entities[idx].alive = false;
            self.log(&format!("The {} dissolves into mist.", name));
        }
    }

    fn try_pickup(&mut self) {
        let px = self.player.x;
        let py = self.player.y;

        if let Some(idx) = self.entities.iter().position(|e| e.x == px && e.y == py && e.alive && e.kind.is_item()) {
            let kind = self.entities[idx].kind;
            self.entities[idx].alive = false;

            match kind {
                EntityKind::Nectar => {
                    let heal = 10.min(self.player.max_hp - self.player.hp);
                    self.player.hp += heal;
                    self.log(&format!("You drink the nectar. +{} HP.", heal));
                }
                EntityKind::Obol => {
                    self.obols += 1;
                    self.log("You pick up an obol. Payment for the ferryman.");
                }
                EntityKind::Moly => {
                    self.player.strength_turns = 15;
                    self.log("You eat the moly. Power surges through you.");
                }
                EntityKind::StygianBlade => {
                    self.player.attack += 2;
                    self.log("You take up the Stygian Blade. +2 attack.");
                }
                _ => {}
            }
        } else {
            self.log("Nothing to pick up here.");
        }
    }

    fn try_descend(&mut self) {
        let px = self.player.x as usize;
        let py = self.player.y as usize;

        if self.map.tiles[py][px] != crate::map::Tile::Stair {
            self.log("There are no stairs here.");
            return;
        }

        if self.depth >= 7 {
            self.game_over = true;
            self.victory = true;
            return;
        }

        self.depth += 1;
        let heal = 5.min(self.player.max_hp - self.player.hp);
        if heal > 0 {
            self.player.hp += heal;
            self.log(&format!("The descent restores you slightly. +{} HP.", heal));
        }
        self.log(&format!("You descend deeper into the meadows. Depth {}.", self.depth));

        self.map = Map::new(self.depth);
        let (px, py) = self.map.rooms[0].center();
        self.player.x = px as i32;
        self.player.y = py as i32;

        self.entities.clear();
        self.shrines.clear();
        for i in 1..self.map.rooms.len() {
            let room = self.map.rooms[i];
            populate_room(&room, self.depth, &mut self.entities, &self.map);
        }
        // Shrine every other floor
        if self.depth % 2 == 0 && self.map.rooms.len() > 3 {
            let shrine_room = &self.map.rooms[self.map.rooms.len() / 2];
            let (sx, sy) = shrine_room.center();
            self.shrines.push(Shrine::new(sx as i32, sy as i32));
        }

        let fov = self.effective_fov();
        compute_fov(&mut self.map, self.player.x, self.player.y, fov);
    }

    fn try_interact(&mut self) {
        let px = self.player.x;
        let py = self.player.y;

        // Check for shrine
        if let Some(idx) = self.shrines.iter().position(|s| s.x == px && s.y == py && !s.used) {
            let shrine = &self.shrines[idx];
            let a_name = shrine.boon_a.name();
            let a_desc = shrine.boon_a.description();
            let b_name = shrine.boon_b.name();
            let b_desc = shrine.boon_b.description();
            self.log(&format!("A shrine of grey stone. Choose your blessing:"));
            self.log(&format!("[A] {} — {}", a_name, a_desc));
            self.log(&format!("[B] {} — {}", b_name, b_desc));
            self.shrine_prompt = Some(idx);
            return;
        }

        self.log("Nothing to interact with here.");
    }

    pub fn choose_shrine_boon(&mut self, choose_a: bool) {
        if let Some(idx) = self.shrine_prompt {
            let boon = if choose_a {
                self.shrines[idx].boon_a
            } else {
                self.shrines[idx].boon_b
            };
            self.shrines[idx].used = true;
            self.shrine_prompt = None;
            self.apply_boon(boon);
        }
    }

    fn apply_boon(&mut self, boon: Boon) {
        match boon {
            Boon::VitalityOfErebus => {
                self.player.max_hp += 10;
                self.player.hp = self.player.max_hp;
                self.log("Erebus grants vitality. +10 max HP, healed to full.");
            }
            Boon::WrathOfAres => {
                self.player.attack += 3;
                self.log("Ares grants fury. +3 attack.");
            }
            Boon::AegisOfAthena => {
                self.player.defense += 2;
                self.log("Athena grants protection. +2 defense.");
            }
            Boon::SwiftnessOfHermes => {
                for ab in &mut self.abilities {
                    ab.cooldown = (ab.cooldown - 3).max(0);
                }
                self.log("Hermes grants swiftness. All cooldowns reduced.");
            }
            Boon::GraceOfPersephone => {
                let heal = 15.min(self.player.max_hp - self.player.hp);
                self.player.hp += heal;
                self.log(&format!("Persephone grants grace. +{} HP.", heal));
            }
            Boon::EyesOfNyx => {
                self.fov_radius += 3;
                self.log("Nyx grants sight. You see further into the dark.");
            }
        }
    }

    fn try_ability(&mut self, idx: usize) {
        if idx >= self.abilities.len() {
            return;
        }
        if !self.abilities[idx].ready() {
            let cd = self.abilities[idx].cooldown;
            let name = self.abilities[idx].ability.name();
            self.log(&format!("{} is on cooldown ({} turns).", name, cd));
            return;
        }

        let ability = self.abilities[idx].ability;
        match ability {
            Ability::Dash => {
                // Dash: we need a direction. For simplicity, dash toward nearest enemy
                // or in the last move direction. Let's dash away from nearest enemy.
                if let Some((dx, dy)) = self.find_dash_direction() {
                    self.abilities[idx].trigger();
                    // Find furthest valid position
                    let (mut fx, mut fy) = (self.player.x, self.player.y);
                    for step in 1..=2 {
                        let sx = self.player.x + dx * step;
                        let sy = self.player.y + dy * step;
                        if self.map.in_bounds(sx, sy) && self.map.tiles[sy as usize][sx as usize].walkable() {
                            fx = sx;
                            fy = sy;
                        } else {
                            break;
                        }
                    }
                    if fx != self.player.x || fy != self.player.y {
                        self.player.x = fx;
                        self.player.y = fy;
                        self.log("You dash through the grey air.");
                    } else {
                        self.log("Nowhere to dash.");
                        self.abilities[idx].cooldown = 0; // refund
                    }
                } else {
                    self.log("No direction to dash.");
                }
            }
            Ability::SpectralScream => {
                self.abilities[idx].trigger();
                let px = self.player.x;
                let py = self.player.y;
                let mut hit = 0;
                for e in &mut self.entities {
                    if !e.alive || !e.kind.is_enemy() { continue; }
                    let dist = ((e.x - px).pow(2) + (e.y - py).pow(2)) as f64;
                    if dist <= 9.0 && self.map.visible[e.y as usize][e.x as usize] {
                        let damage = 6;
                        e.hp -= damage;
                        hit += 1;
                        if e.hp <= 0 {
                            e.alive = false;
                        }
                    }
                }
                if hit > 0 {
                    self.log(&format!("You scream into the void. {} enemies shattered.", hit));
                    // (deaths already handled by hp <= 0 check above)
                } else {
                    self.log("Your scream echoes through empty halls.");
                }
            }
            Ability::LethesTouch => {
                self.abilities[idx].trigger();
                let heal = 15.min(self.player.max_hp - self.player.hp);
                self.player.hp += heal;
                self.log(&format!("You touch the waters of Lethe. You forget your pain. +{} HP.", heal));
            }
        }
    }

    fn find_dash_direction(&self) -> Option<(i32, i32)> {
        // Find nearest visible enemy, dash away from it
        let mut nearest: Option<(f64, i32, i32)> = None;
        for e in &self.entities {
            if !e.alive || !e.kind.is_enemy() { continue; }
            if !self.map.visible[e.y as usize][e.x as usize] { continue; }
            let dist = ((e.x - self.player.x).pow(2) + (e.y - self.player.y).pow(2)) as f64;
            if nearest.is_none() || dist < nearest.unwrap().0 {
                nearest = Some((dist, e.x, e.y));
            }
        }

        if let Some((_, ex, ey)) = nearest {
            // Dash away from enemy
            let dx = (self.player.x - ex).signum();
            let dy = (self.player.y - ey).signum();
            if dx == 0 && dy == 0 {
                Some((1, 0)) // default right
            } else {
                Some((dx, dy))
            }
        } else {
            // No enemies visible, dash right by default
            Some((1, 0))
        }
    }

    fn enemy_turns(&mut self) {
        let px = self.player.x;
        let py = self.player.y;

        for i in 0..self.entities.len() {
            if !self.entities[i].alive || !self.entities[i].kind.is_enemy() {
                continue;
            }

            let ex = self.entities[i].x;
            let ey = self.entities[i].y;

            // Only act if visible
            if !self.map.visible[ey as usize][ex as usize] {
                continue;
            }

            let dist = ((px - ex).abs() + (py - ey).abs()) as f64;

            if dist <= 1.5 {
                // Adjacent: attack player with possible special
                self.enemy_attack(i);
            } else if dist < 8.0 {
                // Chase player
                self.enemy_chase(i, px, py);
            }
        }
    }

    fn enemy_chase(&mut self, i: usize, px: i32, py: i32) {
        let ex = self.entities[i].x;
        let ey = self.entities[i].y;
        let dx = (px - ex).signum();
        let dy = (py - ey).signum();

        let moves = if rand::thread_rng().gen_bool(0.5) {
            [(dx, 0), (0, dy), (dx, dy)]
        } else {
            [(0, dy), (dx, 0), (dx, dy)]
        };

        for (mdx, mdy) in moves {
            let nx = ex + mdx;
            let ny = ey + mdy;
            if mdx == 0 && mdy == 0 { continue; }
            if !self.map.in_bounds(nx, ny) { continue; }
            if !self.map.tiles[ny as usize][nx as usize].walkable() { continue; }
            if self.entities.iter().enumerate().any(|(j, e)| j != i && e.alive && e.kind.is_enemy() && e.x == nx && e.y == ny) {
                continue;
            }
            if nx == px && ny == py { continue; }
            self.entities[i].x = nx;
            self.entities[i].y = ny;
            break;
        }
    }

    fn enemy_attack(&mut self, idx: usize) {
        let mut rng = rand::thread_rng();
        let kind = self.entities[idx].kind;
        let atk = self.entities[idx].attack;
        let def = self.player.defense;
        let damage = (atk - def + rng.gen_range(-1..=2)).max(0);
        let name = self.entities[idx].kind.name();

        // Base attack
        if damage > 0 {
            self.player.hp -= damage;
            self.log(&format!("The {} strikes you for {} damage!", name, damage));
        } else {
            self.log(&format!("The {} attacks but you shrug it off.", name));
        }

        // Special abilities
        match kind {
            EntityKind::Lampad => {
                // Chance to blind
                if rng.gen_ratio(1, 4) && self.blind_turns == 0 {
                    self.blind_turns = 5;
                    self.log("The Lampad's torch flares! You are blinded!");
                }
            }
            EntityKind::Empusa => {
                // Life drain
                if damage > 0 {
                    let drain = (damage / 2).max(1);
                    self.entities[idx].hp = (self.entities[idx].hp + drain).min(self.entities[idx].max_hp);
                    self.log(&format!("The Empusa drains your essence! It heals {}.", drain));
                }
            }
            EntityKind::Eurynomos => {
                // Chance to reduce defense temporarily
                if rng.gen_ratio(1, 5) && self.player.defense > 0 {
                    self.player.defense -= 1;
                    self.log("The Eurynomos strips your protection! -1 defense.");
                }
            }
            _ => {}
        }

        if self.player.hp <= 0 {
            self.player.alive = false;
            self.game_over = true;
            self.killed_by = name.to_string();
        }
    }
}
