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

    pub fn normalised(self) -> Self {
        let magnitude = self.magnitude();
        return Vec2 {
            x: self.x / magnitude,
            y: self.y / magnitude,
        };
    }

    pub fn clamp(mut self, other: Vec2) -> () {
        if self.x > other.x {
            self.x = other.x;
        } else if self.x < 0. {
            self.x = 0.;
        }

        if self.y > other.y {
            self.y = other.y;
        } else if self.y < 0. {
            self.y = 0.;
        }
    }
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
