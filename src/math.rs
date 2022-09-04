#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn min<F: Fn(usize) -> Vector2>(len: usize, f: F) -> Vector2 {
        let mut result = Vector2::new(f64::MAX, f64::MAX);

        for n in 0..len {
            let current = f(n);
            if current.x < result.x {
                result.x = current.x;
            }
            if current.y < result.y {
                result.y = current.y;
            }
        }

        result
    }

    pub fn max<F: Fn(usize) -> Vector2>(len: usize, f: F) -> Vector2 {
        let mut result = Vector2::new(f64::MIN, f64::MIN);

        for n in 0..len {
            let current = f(n);
            if current.x > result.x {
                result.x = current.x;
            }
            if current.y > result.y {
                result.y = current.y;
            }
        }

        result
    }

    pub fn distance(&self, another: &Vector2) -> f64 {
        let dx = (self.x - another.x).powf(2.0);
        let dy = (self.y - another.y).powf(2.0);
        return (dx + dy).sqrt();
    }
}
