use rand::Rng;

pub const MAP_W: usize = 80;
pub const MAP_H: usize = 45;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Stair,
    Asphodel, // decorative flower tile
}

impl Tile {
    pub fn walkable(self) -> bool {
        matches!(self, Tile::Floor | Tile::Stair | Tile::Asphodel)
    }

    pub fn transparent(self) -> bool {
        matches!(self, Tile::Floor | Tile::Stair | Tile::Asphodel)
    }

    pub fn glyph(self) -> char {
        match self {
            Tile::Wall => '█',
            Tile::Floor => '·',
            Tile::Stair => '▼',
            Tile::Asphodel => '✿',
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1
            && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub revealed: Vec<Vec<bool>>,
    pub visible: Vec<Vec<bool>>,
    pub rooms: Vec<Rect>,
    pub depth: u32,
}

impl Map {
    pub fn new(depth: u32) -> Self {
        let mut map = Map {
            tiles: vec![vec![Tile::Wall; MAP_W]; MAP_H],
            revealed: vec![vec![false; MAP_W]; MAP_H],
            visible: vec![vec![false; MAP_W]; MAP_H],
            rooms: Vec::new(),
            depth,
        };
        map.generate();
        map
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let room_count = rng.gen_range(8..16);

        for _ in 0..200 {
            if self.rooms.len() >= room_count {
                break;
            }
            let w = rng.gen_range(4..12);
            let h = rng.gen_range(4..10);
            let x = rng.gen_range(1..MAP_W - w - 1);
            let y = rng.gen_range(1..MAP_H - h - 1);
            let room = Rect::new(x, y, w, h);

            if self.rooms.iter().any(|r| r.intersects(&room)) {
                continue;
            }

            self.carve_room(&room, &mut rng);
            if let Some(prev) = self.rooms.last() {
                let (cx, cy) = room.center();
                let (px, py) = prev.center();
                if rng.gen_bool(0.5) {
                    self.carve_h_tunnel(px, cx, py);
                    self.carve_v_tunnel(py, cy, cx);
                } else {
                    self.carve_v_tunnel(py, cy, px);
                    self.carve_h_tunnel(px, cx, cy);
                }
            }
            self.rooms.push(room);
        }

        // Place stairs in the last room
        if let Some(last) = self.rooms.last() {
            let (sx, sy) = last.center();
            self.tiles[sy][sx] = Tile::Stair;
        }
    }

    fn carve_room(&mut self, room: &Rect, rng: &mut impl Rng) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                self.tiles[y][x] = if rng.gen_ratio(1, 12) {
                    Tile::Asphodel
                } else {
                    Tile::Floor
                };
            }
        }
    }

    fn carve_h_tunnel(&mut self, x1: usize, x2: usize, y: usize) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            if y < MAP_H && x < MAP_W {
                if self.tiles[y][x] == Tile::Wall {
                    self.tiles[y][x] = Tile::Floor;
                }
            }
        }
    }

    fn carve_v_tunnel(&mut self, y1: usize, y2: usize, x: usize) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            if y < MAP_H && x < MAP_W {
                if self.tiles[y][x] == Tile::Wall {
                    self.tiles[y][x] = Tile::Floor;
                }
            }
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < MAP_W && (y as usize) < MAP_H
    }

    pub fn clear_visible(&mut self) {
        for row in &mut self.visible {
            for v in row.iter_mut() {
                *v = false;
            }
        }
    }
}
