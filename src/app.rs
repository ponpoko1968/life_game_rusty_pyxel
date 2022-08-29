use rand::prelude::*;

pub struct App {
    width: u32,
    height: u32,
    generation: u32,
    world: Vec<Vec<bool>>,
}

impl App {
    pub fn init(width: u32, height: u32) {
        pyxel::init(
            width,
            height,
            Some("rusty life game"),
            None,
            None,
            None,
            None,
            None,
        );
        let mut app = App {
            width,
            height,
            generation: pyxel::frame_count(),
            world: vec![vec![false; width as usize]; height as usize],
        };
        app.generate_world();

        pyxel::run(app);
    }
    fn generate_world(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut rng = rand::prelude::thread_rng();
                let r = rng.gen_range(0..=100);
                let _x: usize = x.try_into().unwrap();
                let _y: usize = y.try_into().unwrap();
                self.world[_y][_x] = r < 60
            }
        }
    }

    fn update_world(&mut self) -> Vec<Vec<bool>> {
        let mut new_world = vec![vec![false; self.width as usize]; self.height as usize];
        for x in 0..self.width {
            for y in 0..self.height {
                new_world[y as usize][x as usize] = self.judge_window(x, y);
            }
        }
        new_world
    }
    fn judge_window(&mut self, _x: u32, _y: u32) -> bool {
        let x = _x.try_into().expect("msg");
        let y = _y.try_into().expect("msg");
        let neighbors: [(i64, i64); 8] = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let mut count = 0;
        for neighbor in neighbors {
            let mut xx: i64;
            let mut yy: i64;
            xx = if neighbor.0 < 0 {
                (self.width - 1) as i64
            } else {
                neighbor.0
            };
            xx = if xx >= self.width as i64 { 0 } else { xx };
            yy = if neighbor.1 < 0 {
                (self.height - 1) as i64
            } else {
                neighbor.1
            };
            yy = if yy >= self.height as i64 { 0 } else { yy };
            count += if self.world[yy as usize][xx as usize] {
                1
            } else {
                0
            };
        }
        if !self.world[_y as usize][_x as usize] {
            return count == 3;
        }
        if count <= 1 {
            return false;
        } else if count == 2 || count == 3 {
            return true;
        }
        false
    }
}

impl pyxel::PyxelCallback for App {
    fn update(&mut self) {
        if pyxel::frame_count() > self.generation {
            self.world = self.update_world();
            self.generation = pyxel::frame_count()
        }
    }
    fn draw(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let loc = self.world[y as usize][x as usize];
                pyxel::pset(
                    x as f64,
                    y as f64,
                    if loc {
                        pyxel::COLOR_WHITE
                    } else {
                        pyxel::COLOR_BLACK
                    },
                );
            }
        }
        let s = format!("{}", self.generation);
        pyxel::text(0.0, 0.0,&s, pyxel::COLOR_BROWN);
    }
}
