mod agent;
mod vec;

use rand::Rng;

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;
const ITTERS: i32 = 100;
const RUNNERS: i32 = 100;

fn main() {
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
            i,
            vec::Vec2 { x, y },
            true,
            -1,
        )));
    }

    for i in 0..RUNNERS {
        let x = rng.gen::<f64>() * (WIDTH as f64);
        let y = rng.gen::<f64>() * (HEIGHT as f64);

        agents.push(Box::new(agent::SimpleAgent::new(
            i + ITTERS,
            vec::Vec2 { x, y },
            false,
            -1,
        )));
    }

    // simulation loop
    loop {
        let last_agents = agents.clone();

        for agent in agents.iter_mut() {
            agent.update(0.01, &last_agents, bounds);
        }
    }
}
