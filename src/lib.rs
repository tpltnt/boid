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
}

impl Boid {
    /// Create a new boid.
    fn new() -> Boid {
        return Boid {
            position: cgmath::Vector3::new(1f64, 2f64, 3f64),
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        let b = crate::Boid::new();
        assert_eq!(b.position, cgmath::vec3(1f64, 2f64, 3f64));
    }
}
