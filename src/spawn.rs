use rand::Rng;
use crate::entity::{Entity, EntityKind};
use crate::map::{Map, Rect};

/// Spawn enemies and items for a given depth
pub fn populate_room(room: &Rect, depth: u32, entities: &mut Vec<Entity>, map: &Map) {
    let mut rng = rand::thread_rng();

    // Enemies: more and harder as depth increases
    let max_enemies = (depth as usize / 2) + 2;
    let num_enemies = rng.gen_range(0..=max_enemies.min(4));

    for _ in 0..num_enemies {
        let x = rng.gen_range(room.x1 as i32..room.x2 as i32);
        let y = rng.gen_range(room.y1 as i32..room.y2 as i32);

        // Don't stack entities
        if entities.iter().any(|e| e.x == x && e.y == y) {
            continue;
        }
        if !map.tiles[y as usize][x as usize].walkable() {
            continue;
        }

        let kind = pick_enemy(depth, &mut rng);
        entities.push(Entity::enemy(x, y, kind, depth));
    }

    // Items: rarer
    let num_items = if rng.gen_ratio(1, 3) { 1 } else { 0 };
    for _ in 0..num_items {
        let x = rng.gen_range(room.x1 as i32..room.x2 as i32);
        let y = rng.gen_range(room.y1 as i32..room.y2 as i32);

        if entities.iter().any(|e| e.x == x && e.y == y) {
            continue;
        }
        if !map.tiles[y as usize][x as usize].walkable() {
            continue;
        }

        let kind = pick_item(depth, &mut rng);
        entities.push(Entity::item(x, y, kind));
    }
}

fn pick_enemy(depth: u32, rng: &mut impl Rng) -> EntityKind {
    let roll: u32 = rng.gen_range(0..100);
    match depth {
        1..=2 => {
            if roll < 70 { EntityKind::LostShade }
            else { EntityKind::Lampad }
        }
        3..=4 => {
            if roll < 40 { EntityKind::LostShade }
            else if roll < 75 { EntityKind::Lampad }
            else { EntityKind::Eurynomos }
        }
        _ => {
            if roll < 15 { EntityKind::LostShade }
            else if roll < 40 { EntityKind::Lampad }
            else if roll < 70 { EntityKind::Eurynomos }
            else { EntityKind::Empusa }
        }
    }
}

fn pick_item(depth: u32, rng: &mut impl Rng) -> EntityKind {
    let roll: u32 = rng.gen_range(0..100);
    if depth >= 3 && roll < 10 {
        EntityKind::StygianBlade
    } else if roll < 40 {
        EntityKind::Nectar
    } else if roll < 70 {
        EntityKind::Obol
    } else {
        EntityKind::Moly
    }
}
