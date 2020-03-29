use crate::common::*;
use raqote::{
    AntialiasMode, BlendMode, DrawOptions, DrawTarget, LineCap, LineJoin, Path, PathBuilder,
    SolidSource, Source, StrokeStyle,
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
pub fn draw_point(dt: &mut DrawTarget, a: &Point, color: SolidSource) {
    let mut pb = PathBuilder::new();
    let r = 0.1;
    pb.move_to(a.x + r, a.y);
    pb.arc(a.x, a.y, r, 0.0, 2.0 * std::f32::consts::PI);
    pb.close();
    let path = pb.finish();
    dt.fill(&path, &Source::Solid(color), &DrawOptions::new());
}

fn draw_rpath(dt: &mut DrawTarget, path: &Path, color: SolidSource) {
    dt.stroke(
        path,
        &Source::Solid(color),
        &StrokeStyle {
            width: 0.05,
            cap: LineCap::Round,
            join: LineJoin::Miter,
            ..StrokeStyle::default()
        },
        &DrawOptions::new(),
    );
}

pub fn draw_line(dt: &mut DrawTarget, a: &Point, b: &Point, color: SolidSource) {
    let mut pb = PathBuilder::new();
    pb.move_to(a.x, a.y);
    pb.line_to(b.x, b.y);
    draw_rpath(dt, &pb.finish(), color);
}

pub fn draw_path(dt: &mut DrawTarget, points: &Vec<Point>, color: SolidSource) {
    if points.is_empty() {
        return;
    }
    let first = points[0];
    let mut pb = PathBuilder::new();
    pb.move_to(first.x, first.y);
    for point in points {
        draw_point(dt, point, color);
        pb.line_to(point.x, point.y);
    }
    draw_rpath(dt, &pb.finish(), color);
}

pub fn fill_rect(dt: &mut DrawTarget, lb: &Point, rt: &Point, color: SolidSource) {
    dt.fill_rect(
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
pub fn fill_part(dt: &mut DrawTarget, left_x: f32, right_x: f32, color: SolidSource) {
    fill_rect(
        dt,
        &Point::new(left_x, 0.),
        &Point::new(right_x, MAX_Y),
        color,
    );
}

pub fn draw_vertical_line(dt: &mut DrawTarget, x: f32, color: SolidSource) {
    draw_line(dt, &Point::new(x, 0.), &Point::new(x, MAX_Y), color);
}

pub fn draw_borders(dt: &mut DrawTarget, borders: &HorBorders) {
    fill_part(dt, borders.l - 0.1, borders.r + 0.1, GREEN_COLOR);
}
