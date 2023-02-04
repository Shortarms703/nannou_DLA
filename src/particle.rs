use nannou::prelude::*;
use std::collections::HashMap;
// use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Particle {
    pos: Vec2,
    radius: f32,
}

#[derive(Debug)]
pub struct Sections {
    sections: HashMap<Vec<i32>, Section>,
    size: f32,
}

#[derive(Debug)]
struct Section {
    particles: Vec<Particle>,
}

impl Sections {
    pub fn new(size: f32) -> Self {
        Sections {
            sections: HashMap::new(),
            size,
        }
    }

    pub fn insert(&mut self, particle: Particle) {
        let row = (particle.get_pos().x / self.size).floor() as i32;
        let col = (particle.get_pos().y / self.size).floor() as i32;
        match self.sections.get_mut(&vec![row, col]) {
            Some(section) => {
                section.particles.push(particle);
            }
            None => {
                self.sections.insert(
                    vec![row, col],
                    Section::new(vec![particle])
                );
            }
        }
    }

    pub fn collision(&self, particle: &Particle) -> bool {
        // let start = Instant::now();
        for section in self.get_surrounding_sections(particle) {
            if section.collision(particle) {
                return true;
            }
        }
        // println!("Sections.collision: {:?}", start.elapsed());
        false
    }

    fn get_section_key(&self, particle: &Particle) -> (i32, i32) {
        let row = (particle.get_pos().x / self.size).floor() as i32;
        let col = (particle.get_pos().y / self.size).floor() as i32;
        (row, col)
    }
    
    fn get_surrounding_sections(&self, particle: &Particle) -> Vec<&Section> {

        let center_key = self.get_section_key(particle);
        let mut surrounding_sections = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                // let start = Instant::now();
                // self.sections.get(&vec![center_key.0 + x, center_key.1 + y];);
                // &vec![center_key.0 + x, center_key.1 + y];
                // let start2 = Instant::now();
                center_key.0;
                // println!("Sections.get_surrounding_sections addition 1: {:?}", start2.elapsed());
                // let start2 = Instant::now();
                center_key.1;
                // println!("Sections.get_surrounding_sections addition 2: {:?}", start2.elapsed());
                [center_key.0 + x, center_key.1 + y];
                // println!("Sections.get_surrounding_sections match: {:?}", start.elapsed());
                // println!("Sections.get_surrounding_sections match: {:?}, {:?}", start.elapsed(), [center_key.0 + x, center_key.1 + y]);
                match self.sections.get(&vec![center_key.0 + x, center_key.1 + y]) {
                    Some(section) => {
                        surrounding_sections.push(section);
                    }
                    None => (),
                }
            }
        }
        // println!("Sections.get_surrounding_sections: {:?}", start.elapsed());
        surrounding_sections
    }

    pub fn all_particles(&self) -> Vec<&Particle> {
        let mut particles = vec![];
        for section in self.sections.values() {
            for particle in section.particles.iter() {
                particles.push(particle);
            }
        }
        particles
    }
}

impl Section {
    fn new(particles: Vec<Particle>) -> Self {
        Section {
            particles
        }
    }

    fn collision(&self, particle: &Particle) -> bool {
        particle.collision(&self.particles)
    }
}

impl Particle {
    pub fn new(x: f32, y: f32, radius: f32) -> Particle {
        Particle {
            pos: vec2(x, y),
            radius
        }
    }

    pub fn new_on_radius(radius: f32, particle_radius: f32) -> Particle {
        let angle = random::<f32>() * PI * 2.;
        Particle::new(radius * angle.cos(), radius * angle.sin(), particle_radius)
    }

    pub fn update(&mut self, win: Rect, move_speed: f32) {
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

    pub fn collision(&self, points: &Vec<Particle>) -> bool {
        for p in points {
            if p.pos.distance_squared(self.pos) < self.radius * self.radius {
                return true;
            }
        }
        false
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}
