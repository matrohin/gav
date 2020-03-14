use crate::common::Point;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
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
    let path = pb.finish();
    dt.stroke(
        &path,
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
