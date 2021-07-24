use std::ops;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalised(&self) -> Self {
        let magnitude = self.magnitude();
        return Vec2 {
            x: self.x / magnitude,
            y: self.y / magnitude,
        };
    }

    pub fn wrap(&self, bounds: Vec2) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        if x > bounds.x {
            x -= bounds.x;
        } else if x < 0. {
            x += bounds.x;
        }

        if y > bounds.y {
            y -= bounds.y;
        } else if y < 0. {
            y += bounds.y;
        }

        Vec2 { x: x, y: y }
    }
}

pub fn get_shortest_wrapped_path(a: Vec2, b: Vec2, bounds: Vec2) -> Vec2 {
    let x1 = a.x - b.x;
    let x2 = a.x - b.x - bounds.x;
    let x3 = a.x - b.x + bounds.x;

    let dx;
    if x1.abs() < x2.abs() && x1.abs() < x3.abs() {
        dx = x1;
    } else if x2.abs() < x3.abs() {
        dx = x2;
    } else {
        dx = x3;
    }

    let y1 = a.y - b.y;
    let y2 = a.y - b.y - bounds.y;
    let y3 = a.y - b.y + bounds.y;

    let dy;
    if y1.abs() < y2.abs() && y1.abs() < y3.abs() {
        dy = y1;
    } else if y2.abs() < y3.abs() {
        dy = y2;
    } else {
        dy = y3;
    }

    return Vec2 { x: dx, y: dy };
}

impl ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, s: f64) -> Self {
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}
