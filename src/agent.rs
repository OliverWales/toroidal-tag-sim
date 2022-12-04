use crate::vec;

// Agent trait as interface for agent behaviour. Simple agent as concrete
// implementation of this. NB: Vec<Box<dyn ...>> cloning implementation based on
// https://stackoverflow.com/questions/50017987/cant-clone-vecboxtrait-because-trait-cannot-be-made-into-an-object

pub trait Agent: AgentClone + Send + Sync {
    fn get_id(&self) -> i32;
    fn get_position(&self) -> vec::Vec2;
    fn is_it(&self) -> bool;
    fn last_itted_by(&self) -> i32;
    fn update(&mut self, delta_t: f64, neighbours: &[Box<dyn Agent>], options: Options);
}

pub trait AgentClone {
    fn clone_box(&self) -> Box<dyn Agent>;
}

impl<T: 'static + Agent + Clone> AgentClone for T {
    fn clone_box(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Agent> {
    fn clone(&self) -> Box<dyn Agent> {
        self.clone_box()
    }
}

pub struct Options {
    pub bounds: vec::Vec2,
    pub speed: f64,
    pub it_range: f64,
}

#[derive(Clone)]
pub struct SimpleAgent {
    pub id: i32,
    pub pos: vec::Vec2,
    pub it: bool,
    pub last_itted: i32,
    pub last_itted_by: i32,
}

impl SimpleAgent {
    pub fn new(id: i32, pos: vec::Vec2, it: bool) -> Self {
        SimpleAgent {
            id,
            pos,
            it,
            last_itted: -1,
            last_itted_by: -1,
        }
    }
}

impl Agent for SimpleAgent {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_position(&self) -> vec::Vec2 {
        self.pos
    }

    fn is_it(&self) -> bool {
        self.it
    }

    fn last_itted_by(&self) -> i32 {
        self.last_itted_by
    }

    fn update(&mut self, delta_t: f64, neighbours: &[Box<dyn Agent>], options: Options) {
        // check if an "it" has occurred
        if self.it {
            // check for runner within range that is not the one that last itted you
            for neighbour in neighbours {
                if !neighbour.is_it()
                    && neighbour.get_id() != self.id
                    && self.last_itted_by != neighbour.get_id()
                {
                    let dist = vec::get_shortest_wrapped_path(
                        neighbour.get_position(),
                        self.pos,
                        options.bounds,
                    )
                    .magnitude();
                    if dist <= options.it_range {
                        // it the neighbour
                        self.last_itted = neighbour.get_id();
                        self.it = false;
                        continue; // only allow an agent to it once per update
                    }
                }
            }
        } else {
            // check for itter within range that is not the one that you itted last
            for neighbour in neighbours {
                if neighbour.is_it()
                    && neighbour.get_id() != self.id
                    && self.last_itted != neighbour.get_id()
                {
                    let dist = vec::get_shortest_wrapped_path(
                        neighbour.get_position(),
                        self.pos,
                        options.bounds,
                    )
                    .magnitude();
                    if dist <= options.it_range {
                        // itted by the neighbour
                        self.last_itted_by = neighbour.get_id();
                        self.it = true;
                        continue; // only allow an agent to be itted once per update
                    }
                }
            }
        }

        if self.it {
            // if it, find closest runner and chase towards them
            let mut min_dist = std::f64::MAX;
            let mut target = None;

            for neighbour in neighbours {
                if !neighbour.is_it()
                    && neighbour.get_id() != self.id
                    && self.last_itted_by != neighbour.get_id()
                {
                    let dist = vec::get_shortest_wrapped_path(
                        neighbour.get_position(),
                        self.pos,
                        options.bounds,
                    )
                    .magnitude();
                    if dist < min_dist {
                        min_dist = dist;
                        target = Some(neighbour);
                    }
                }
            }

            if let Some(t) = target {
                self.pos +=
                    vec::get_shortest_wrapped_path(t.get_position(), self.pos, options.bounds)
                        .normalised()
                        * delta_t
                        * options.speed
                        * 1.1; // small speed boost given to itters
                self.pos = self.pos.wrap(options.bounds);
            };
        } else {
            // if not it, find closest itter and run away from them
            let mut min_dist = std::f64::MAX;
            let mut assailant = None;

            for neighbour in neighbours {
                if neighbour.is_it()
                    && neighbour.get_id() != self.id
                    && self.last_itted != neighbour.get_id()
                {
                    let dist = vec::get_shortest_wrapped_path(
                        neighbour.get_position(),
                        self.pos,
                        options.bounds,
                    )
                    .magnitude();
                    if dist < min_dist {
                        min_dist = dist;
                        assailant = Some(neighbour);
                    }
                }
            }

            if let Some(a) = assailant {
                self.pos +=
                    vec::get_shortest_wrapped_path(a.get_position(), self.pos, options.bounds)
                        .normalised()
                        * -delta_t
                        * options.speed;
                self.pos = self.pos.wrap(options.bounds);
            }
        }
    }
}
