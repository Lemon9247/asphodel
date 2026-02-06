use crate::map::{Map, MAP_W, MAP_H};

/// Simple raycasting FOV - cast rays in all directions
pub fn compute_fov(map: &mut Map, px: i32, py: i32, radius: i32) {
    map.clear_visible();

    let r2 = (radius * radius) as f64;
    let rays = 360;

    for i in 0..rays {
        let angle = (i as f64) * std::f64::consts::PI * 2.0 / (rays as f64);
        let dx = angle.cos();
        let dy = angle.sin();

        let mut x = px as f64 + 0.5;
        let mut y = py as f64 + 0.5;

        for _ in 0..radius {
            let ix = x as i32;
            let iy = y as i32;

            if ix < 0 || iy < 0 || ix >= MAP_W as i32 || iy >= MAP_H as i32 {
                break;
            }

            let dist = ((ix - px) as f64).powi(2) + ((iy - py) as f64).powi(2);
            if dist > r2 {
                break;
            }

            let ux = ix as usize;
            let uy = iy as usize;
            map.visible[uy][ux] = true;
            map.revealed[uy][ux] = true;

            if !map.tiles[uy][ux].transparent() {
                break;
            }

            x += dx;
            y += dy;
        }
    }
}
