use crate::common::*;
use raqote::{
    AntialiasMode, BlendMode, DrawOptions, DrawTarget, LineCap, LineJoin, Path, PathBuilder,
    SolidSource, Source, StrokeStyle, Transform,
};

pub const WHITE_COLOR: SolidSource = SolidSource {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    a: 0xff,
};
pub const BLUE_COLOR: SolidSource = SolidSource {
    r: 0,
    g: 0,
    b: 0xff,
    a: 0xff,
};
pub const GREEN_COLOR: SolidSource = SolidSource {
    r: 0,
    g: 0xff,
    b: 0,
    a: 0xff,
};
pub const YELLOW_COLOR: SolidSource = SolidSource {
    r: 0xff,
    g: 0xff,
    b: 0,
    a: 0xff,
};
pub const RED_COLOR: SolidSource = SolidSource {
    r: 0xff,
    g: 0,
    b: 0,
    a: 0xff,
};

pub struct DrawContext {
    dt: DrawTarget,
    draw_width: f32,
}

impl DrawContext {
    pub fn new(size: (usize, usize), draw_width: f32) -> Self {
        let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
        let transform = Transform::create_translation(1., -MAX_Y - 1.);
        let transform = transform.post_scale(
            (size.0 as f32) / (MAX_X + 2.0),
            -(size.0 as f32) / (MAX_Y + 2.0),
        );
        dt.set_transform(&transform);
        Self { dt, draw_width }
    }

    pub fn clear(self: &mut Self) {
        self.dt
            .clear(SolidSource::from_unpremultiplied_argb(0, 0, 0, 0xff));
    }

    pub fn get_data(self: &Self) -> &[u32] {
        self.dt.get_data()
    }

    pub fn draw_point(self: &mut Self, a: &Point, color: SolidSource) {
        let mut pb = PathBuilder::new();
        let r = self.draw_width;
        pb.move_to(a.x + r, a.y);
        pb.arc(a.x, a.y, r, 0.0, 2.0 * std::f32::consts::PI);
        pb.close();
        let path = pb.finish();
        self.dt
            .fill(&path, &Source::Solid(color), &DrawOptions::new());
    }

    fn draw_rpath(self: &mut Self, path: &Path, color: SolidSource) {
        self.dt.stroke(
            path,
            &Source::Solid(color),
            &StrokeStyle {
                width: self.draw_width / 2.0,
                cap: LineCap::Round,
                join: LineJoin::Miter,
                ..StrokeStyle::default()
            },
            &DrawOptions::new(),
        );
    }

    pub fn draw_line(self: &mut Self, a: &Point, b: &Point, color: SolidSource) {
        let mut pb = PathBuilder::new();
        pb.move_to(a.x, a.y);
        pb.line_to(b.x, b.y);
        self.draw_rpath(&pb.finish(), color);
    }

    pub fn draw_path(self: &mut Self, points: &Vec<Point>, color: SolidSource) {
        if points.is_empty() {
            return;
        }
        let first = points[0];
        let mut pb = PathBuilder::new();
        pb.move_to(first.x, first.y);
        for point in points {
            self.draw_point(point, color);
            pb.line_to(point.x, point.y);
        }
        self.draw_rpath(&pb.finish(), color);
    }

    pub fn fill_rect(self: &mut Self, lb: &Point, rt: &Point, color: SolidSource) {
        self.dt.fill_rect(
            lb.x,
            lb.y,
            rt.x - lb.x,
            rt.y - lb.y,
            &Source::Solid(color),
            &DrawOptions {
                blend_mode: BlendMode::SrcOut,
                alpha: 0.3,
                antialias: AntialiasMode::Gray,
            },
        )
    }
    pub fn fill_part(self: &mut Self, left_x: f32, right_x: f32, color: SolidSource) {
        self.fill_rect(&Point::new(left_x, 0.), &Point::new(right_x, MAX_Y), color);
    }

    pub fn draw_vertical_line(self: &mut Self, x: f32, color: SolidSource) {
        self.draw_line(&Point::new(x, 0.), &Point::new(x, MAX_Y), color);
    }

    pub fn draw_borders(self: &mut Self, borders: &HorBorders) {
        self.fill_part(borders.l - 0.1, borders.r + 0.1, GREEN_COLOR);
    }
}
