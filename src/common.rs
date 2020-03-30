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

pub fn cmp_by_xy(a: &Point, b: &Point) -> std::cmp::Ordering {
    cmp_by_x(a, b).then_with(|| cmp_by_y(a, b))
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

#[derive(Copy, Clone, Debug)]
pub struct IndexBorders {
    pub l: usize,
    pub r: usize,
}

impl IndexBorders {
    pub fn left(&self) -> Self {
        IndexBorders {
            l: self.l,
            r: (self.l + self.r) / 2,
        }
    }
    pub fn right(&self) -> Self {
        IndexBorders {
            l: (self.l + self.r) / 2,
            r: self.r,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HorBorders {
    pub l: f32,
    pub r: f32,
}

impl HorBorders {
    pub fn new(l: f32, r: f32) -> Self {
        Self { l, r }
    }
    pub fn from_indexes(points: &Vec<Point>, borders: &IndexBorders) -> Self {
        Self {
            l: points[borders.l].x,
            r: points[borders.r - 1].x,
        }
    }
}
