use nannou::prelude::*;

#[derive(Clone)]
#[derive(Debug)]
pub(crate) struct Particle {
    pos: Vec2,
}

impl Particle {
    pub(crate) fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: vec2(x, y),
        }
    }

    pub(crate) fn new_on_radius(radius: f32) -> Particle {
        let angle = random::<f32>() * PI * 2.;
        Particle::new(radius * angle.cos(), radius * angle.sin())
    }

    pub(crate) fn update(&mut self, win: Rect, move_speed: f32) {
        if random::<f32>() < 0.5 {
            self.pos.x += move_speed;
        } else {
            self.pos.x -= move_speed;
        }
        if random::<f32>() < 0.5 {
            self.pos.y += move_speed;
        } else {
            self.pos.y -= move_speed;
        }
        self.pos.x = clamp(self.pos.x, win.right(), win.left());
        self.pos.y = clamp(self.pos.y, win.top(), win.bottom());
    }

    pub(crate) fn collision(&mut self, points: &Vec<Particle>, radius: f32) -> bool {
        for p in points {
            if p.pos.distance_squared(self.pos) < radius * radius {
                return true;
            }
        }
        false
    }
}

impl Particle {
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
}