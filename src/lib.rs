/*!
boid
====
**boid** is a 2 and 3-dimensional boid library written in the Rust programming language.
Its purpose is to implement flocking behaviour.
*/

extern crate cgmath;

/// The data structure to store relevant information
/// for each boid.
#[derive(Debug)]
struct Boid {
    pub position: cgmath::Vector3<f64>,
    pub velocity: cgmath::Vector3<f64>,
    pub yaw: f64,       // rotation in X-Y plane
    pub pitch: f64,     // rotation in X-Z plane
    pub timedelta: f64, //time (to be) passed between updates
}

impl Boid {
    /// Create a new boid with all values set to zero.
    pub fn new() -> Boid {
        return Boid {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            velocity: cgmath::Vector3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            timedelta: 0.0,
        };
    }

    /// Update position based on time passed since last update.
    pub fn update(&mut self) {
        self.position += self.timedelta * self.velocity;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        let b = crate::Boid::new();
        assert_eq!(b.position, cgmath::vec3(0.0, 0.0, 0.0));
        assert_eq!(b.velocity, cgmath::vec3(0.0, 0.0, 0.0));
        assert_eq!(b.yaw, 0.0);
        assert_eq!(b.pitch, 0.0);
        assert_eq!(b.timedelta, 0.0);
    }

    #[test]
    fn update() {
        let mut b = crate::Boid::new();
        b.update();
        assert_eq!(b.position, cgmath::vec3(0.0, 0.0, 0.0));
        assert_eq!(b.velocity, cgmath::vec3(0.0, 0.0, 0.0));
        assert_eq!(b.yaw, 0.0);
        assert_eq!(b.pitch, 0.0);
        assert_eq!(b.timedelta, 0.0);
    }
}
