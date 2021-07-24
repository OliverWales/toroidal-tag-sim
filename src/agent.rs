use crate::vec;

const IT_DIST: f64 = 1.;

// NB: Vec<Box<dyn ...>> cloning implementation based on
// https://stackoverflow.com/questions/50017987/cant-clone-vecboxtrait-because-trait-cannot-be-made-into-an-object

pub trait Agent: AgentClone {
    fn get_id(&self) -> i32;
    fn get_position(&self) -> vec::Vec2;
    fn is_it(&self) -> bool;
    fn last_itted_by(&self) -> i32;
    fn update(&mut self, delta_t: f64, neighbours: &Vec<Box<dyn Agent>>, bounds: vec::Vec2) -> ();
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

#[derive(Clone)]
pub struct SimpleAgent {
    pub id: i32,
    pub pos: vec::Vec2,
    pub it: bool,
    pub last_itted_by: i32,
}

impl SimpleAgent {
    pub fn new(id: i32, pos: vec::Vec2, it: bool, last_itted_by: i32) -> Self {
        SimpleAgent {
            id,
            pos,
            it,
            last_itted_by,
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

    fn update(&mut self, delta_t: f64, neighbours: &Vec<Box<dyn Agent>>, bounds: vec::Vec2) {
        // check if an "it" has occurred
        if self.it {
            // check for runner within range that is not the one that itted you
            for neighbour in neighbours {
                if !neighbour.is_it() && self.last_itted_by != neighbour.get_id() {
                    let dist = (neighbour.get_position() - self.pos).magnitude();
                    if dist <= IT_DIST {
                        self.it = false;
                        println!("{} itted {}", self.id, neighbour.get_id());
                        break;
                    }
                }
            }
        } else {
            // check for itter within range that is not the one that you itted
            for neighbour in neighbours {
                if neighbour.is_it() && neighbour.last_itted_by() != self.id {
                    let dist = (neighbour.get_position() - self.pos).magnitude();
                    if dist <= IT_DIST {
                        self.it = true;
                        self.last_itted_by = neighbour.get_id();
                        println!("{} was itted by {}", self.id, neighbour.get_id());
                        break;
                    }
                }
            }
        }

        // update according to current "it" status
        if self.it {
            // find closest runner and go towards them
            let mut min_dist = std::f64::MAX;
            let mut target = None;

            for neighbour in neighbours {
                if !neighbour.is_it() {
                    let dist = (neighbour.get_position() - self.pos).magnitude();
                    if dist < min_dist {
                        min_dist = dist;
                        target = Some(neighbour);
                    }
                }
            }

            if target.is_some() {
                self.pos += (target.unwrap().get_position() - self.pos).normalised() * delta_t;
                self.pos.clamp(bounds);
            };
        } else {
            // find closest iter and go away from them
            let mut min_dist = std::f64::MAX;
            let mut assailant = None;

            for neighbour in neighbours {
                let dist = (neighbour.get_position() - self.pos).magnitude();
                if dist < min_dist {
                    min_dist = dist;
                    assailant = Some(neighbour);
                }
            }

            if assailant.is_some() {
                self.pos += (assailant.unwrap().get_position() - self.pos).normalised() * -delta_t;
                self.pos.clamp(bounds);
            };
        }
    }
}
