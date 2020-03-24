use euclid::Point2D;

pub const MAX_X: f32 = 30.0;
pub const MAX_Y: f32 = 30.0;

pub type Point = Point2D<f32, f32>;

pub fn rotation(a: &Point, b: &Point, c: &Point) -> f32 {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)
}

pub fn cmp_by_x(a: &Point, b: &Point) -> std::cmp::Ordering {
    a.x.partial_cmp(&b.x).unwrap()
}

pub fn cmp_by_y(a: &Point, b: &Point) -> std::cmp::Ordering {
    a.y.partial_cmp(&b.y).unwrap()
}

#[derive(Copy, Clone, Debug)]
pub struct Pair {
    pub a: Point,
    pub b: Point,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl Eq for Pair {}

impl Pair {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
    pub fn inf() -> Self {
        Self {
            a: Point::new(0., 0.),
            b: Point::new(std::f32::INFINITY, std::f32::INFINITY),
        }
    }
    pub fn square_len(&self) -> f32 {
        (self.a - self.b).square_length()
    }
}
