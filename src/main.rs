mod particle;

// use std::time::Instant;
use particle::{Particle, Sections};
use nannou::prelude::*;


fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    particle_radius: f32,
    particle_move_speed: f32,
    spawn_radius: f32,
    alive_particle_limit: usize,
    dead_particles: Sections,
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(100.0));
    println!("{:?}", app.loop_mode());
    app.new_window().size(1200, 1200).view(view).build().unwrap();

    Model {
        particle_radius: 10.,
        particle_move_speed: 3.,
        spawn_radius: 600.,
        alive_particle_limit: 25,
        dead_particles: {
            let mut sections = Sections::new(10.);
            sections.insert(Particle::new(0., 0., 10.));
            sections
        },
        particles: vec![],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // for _ in 0..100 {
        for _ in 0..(model.alive_particle_limit - model.particles.len()) {
            let p = Particle::new_on_radius(model.spawn_radius, model.particle_radius);
            model.particles.push(p);
        }

        let mut removed = vec![];
        while removed.len() == 0 {
            // let start = Instant::now();
            for e in model.particles.iter_mut().enumerate() {
                let (n, p): (usize, &mut Particle) = e;

                // let start2 = Instant::now();
                let collision = model.dead_particles.collision(p);
                // println!("outside Sections.collision: {:?}", start2.elapsed());

                let all_particles = model.dead_particles.all_particles();
                let mut b = vec![];
                for p in all_particles {
                    b.push(p.clone());
                }
                // let start3 = Instant::now();
                p.collision(&b);
                // println!("Particle.collision: {:?}", start3.elapsed());

                if collision {
                    model.dead_particles.insert(p.clone());
                    removed.push(n);
                } else {
                    let win = app.window_rect();
                    p.update(win, model.particle_move_speed);
                }
            }
            // println!("collision check: {:?}", start.elapsed());
        }
        for removed_point_index in removed {
            model.particles.remove(removed_point_index);
            model.particles.push(Particle::new_on_radius(model.spawn_radius, model.particle_radius));
        }
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    app.main_window().capture_frame(format!("frames/{:03}.png", frame.nth()));
    let draw = app.draw();

    draw.background().color(BLACK);
    // for p in &model.particles {
    //     draw.ellipse().x_y(p.get_pos().x, p.get_pos().y).w_h(p.get_radius(), p.get_radius()).color(WHITE);
    // }
    for p in model.dead_particles.all_particles() {
        draw.ellipse().x_y(p.get_pos().x, p.get_pos().y).w_h(p.get_radius(), p.get_radius()).color(AQUA);
    }

    draw_fps(app, &draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_fps(app: &App, draw: &Draw) {
    let fps = app.fps();
    let win = app.window_rect();
    let r = Rect::from_w_h(75.0, 15.0).top_left_of(win);
    // draw.rect()
    //     .xy(r.xy())
    //     .wh(r.wh())
    //     .color(DIMGREY);
    // draw.text(&format!("fps: {:.2}", fps))
    //     .xy(r.xy())
    //     .color(WHITE);
}