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
