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
    position: cgmath::Vector3<f64>,
    velocity: cgmath::Vector3<f64>,
    yaw: f64,   // rotation in X-Y plane
    pitch: f64, // rotation in X-Z plane
}

impl Boid {
    /// Create a new boid with all values set to zero.
    fn new() -> Boid {
        return Boid {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            velocity: cgmath::Vector3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        };
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
    }
}
