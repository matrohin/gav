use euclid::Point2D;

pub type Point = Point2D<f32, f32>;

pub fn rotation(a: &Point, b: &Point, c: &Point) -> f32 {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)
}
