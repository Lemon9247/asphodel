use rand::Rng;
use crate::entity::{Entity, EntityKind};
use crate::fov::compute_fov;
use crate::map::Map;
use crate::spawn::populate_room;

const FOV_RADIUS: i32 = 8;
const LOG_SIZE: usize = 8;

pub enum Action {
    Move(i32, i32),
    Wait,
    Pickup,
    Descend,
}

pub struct Game {
    pub map: Map,
    pub player: Entity,
    pub entities: Vec<Entity>,
    pub log: Vec<String>,
    pub depth: u32,
    pub obols: u32,
    pub turns: u32,
    pub game_over: bool,
    pub victory: bool,
}

impl Game {
    pub fn new() -> Self {
        let depth = 1;
        let map = Map::new(depth);
        let (px, py) = map.rooms[0].center();
        let player = Entity::player(px as i32, py as i32);

        let mut entities = Vec::new();
        // Populate all rooms except the first (player spawn)
        for i in 1..map.rooms.len() {
            let room = map.rooms[i];
            populate_room(&room, depth, &mut entities, &map);
        }

        let mut game = Game {
            map, player, entities,
            log: Vec::new(),
            depth, obols: 0, turns: 0,
            game_over: false, victory: false,
        };
        compute_fov(&mut game.map, game.player.x, game.player.y, FOV_RADIUS);
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
        }

        // Tick buffs
        if self.player.strength_turns > 0 {
            self.player.strength_turns -= 1;
            if self.player.strength_turns == 0 {
                self.log("The moly's power fades.");
            }
        }

        // Enemy turns
        self.enemy_turns();
        self.turns += 1;

        // Recompute FOV
        compute_fov(&mut self.map, self.player.x, self.player.y, FOV_RADIUS);
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
            // Victory!
            self.game_over = true;
            self.victory = true;
            self.log("You descend past the final meadow.");
            self.log("The river Lethe gleams below. You could drink and forget.");
            self.log("Instead, you remember. You choose to remember.");
            self.log("You have reached Elysium.");
            return;
        }

        self.depth += 1;
        self.log(&format!("You descend deeper into the meadows. Depth {}.", self.depth));

        self.map = Map::new(self.depth);
        let (px, py) = self.map.rooms[0].center();
        self.player.x = px as i32;
        self.player.y = py as i32;

        self.entities.clear();
        for i in 1..self.map.rooms.len() {
            let room = self.map.rooms[i];
            populate_room(&room, self.depth, &mut self.entities, &self.map);
        }

        compute_fov(&mut self.map, self.player.x, self.player.y, FOV_RADIUS);
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

            // Only act if visible (player can see them = they can see player)
            if !self.map.visible[ey as usize][ex as usize] {
                continue;
            }

            let dist = ((px - ex).abs() + (py - ey).abs()) as f64;

            if dist <= 1.5 {
                // Adjacent: attack player
                self.enemy_attack(i);
            } else if dist < 8.0 {
                // Chase player (simple movement toward)
                let dx = (px - ex).signum();
                let dy = (py - ey).signum();

                // Try primary direction, then fallback
                let moves = if rand::thread_rng().gen_bool(0.5) {
                    [(dx, 0), (0, dy), (dx, dy)]
                } else {
                    [(0, dy), (dx, 0), (dx, dy)]
                };

                for (mdx, mdy) in moves {
                    let nx = ex + mdx;
                    let ny = ey + mdy;
                    if mdx == 0 && mdy == 0 {
                        continue;
                    }
                    if !self.map.in_bounds(nx, ny) {
                        continue;
                    }
                    if !self.map.tiles[ny as usize][nx as usize].walkable() {
                        continue;
                    }
                    // Don't walk on other enemies
                    if self.entities.iter().enumerate().any(|(j, e)| j != i && e.alive && e.kind.is_enemy() && e.x == nx && e.y == ny) {
                        continue;
                    }
                    // Don't walk onto player (that's what attack is for)
                    if nx == px && ny == py {
                        continue;
                    }
                    self.entities[i].x = nx;
                    self.entities[i].y = ny;
                    break;
                }
            }
        }
    }

    fn enemy_attack(&mut self, idx: usize) {
        let mut rng = rand::thread_rng();
        let atk = self.entities[idx].attack;
        let def = self.player.defense;
        let damage = (atk - def + rng.gen_range(-1..=2)).max(0);
        let name = self.entities[idx].kind.name();

        if damage > 0 {
            self.player.hp -= damage;
            self.log(&format!("The {} strikes you for {} damage!", name, damage));
        } else {
            self.log(&format!("The {} attacks but you shrug it off.", name));
        }

        if self.player.hp <= 0 {
            self.player.alive = false;
            self.game_over = true;
            self.log("You dissolve. Another shade, forgotten.");
            self.log(&format!("Depth: {} | Obols: {} | Turns: {}", self.depth, self.obols, self.turns));
            self.log("[Press any key to quit]");
        }
    }
}
