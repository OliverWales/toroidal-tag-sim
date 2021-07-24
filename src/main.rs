extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

mod agent;
mod vec;

// config
const WIDTH: u32 = 1366;
const HEIGHT: u32 = 768;
const ITTERS: u32 = 50;
const RUNNERS: u32 = 1000;
const IT_DIST: f64 = 4.;

// colours
const BG_COL: [f32; 4] = [0.1, 0.15, 0.2, 1.0];
const ITTER_COL: [f32; 4] = [1.0, 0.2, 0.4, 1.0];
const RUNNER_COL: [f32; 4] = [0.8, 0.8, 0.8, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        format!(
            "Tag simulator {}x{} ({} taggers, {} runners)",
            WIDTH, HEIGHT, ITTERS, RUNNERS
        ),
        [WIDTH, HEIGHT],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut rng = rand::thread_rng();
    let mut agents: Vec<Box<dyn agent::Agent>> = Vec::new();
    let bounds = vec::Vec2 {
        x: WIDTH as f64,
        y: HEIGHT as f64,
    };

    for i in 0..ITTERS {
        let x = rng.gen::<f64>() * (WIDTH as f64);
        let y = rng.gen::<f64>() * (HEIGHT as f64);

        agents.push(Box::new(agent::SimpleAgent::new(
            i as i32,
            vec::Vec2 { x, y },
            true,
            -1,
            -1,
        )));
    }

    for i in 0..RUNNERS {
        let x = rng.gen::<f64>() * (WIDTH as f64);
        let y = rng.gen::<f64>() * (HEIGHT as f64);

        agents.push(Box::new(agent::SimpleAgent::new(
            (i + ITTERS) as i32,
            vec::Vec2 { x, y },
            false,
            -1,
            -1,
        )));
    }

    // simulation loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |_c, gl| {
                graphics::clear(BG_COL, gl);
            });

            for agent in &agents {
                let player = graphics::ellipse::circle(
                    agent.get_position().x,
                    agent.get_position().y,
                    IT_DIST / 2.,
                );

                gl.draw(args.viewport(), |c, gl| {
                    let transform = c.transform;
                    if agent.is_it() {
                        graphics::ellipse(ITTER_COL, player, transform, gl);
                    } else {
                        graphics::ellipse(RUNNER_COL, player, transform, gl);
                    }
                })
            }
        }

        if let Some(args) = e.update_args() {
            let mut last_agents = agents.clone();

            for agent in &mut agents {
                agent.update(args.dt, &mut last_agents, IT_DIST, bounds);
            }
        }
    }
}
