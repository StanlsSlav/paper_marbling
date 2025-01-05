use core::f32::consts::TAU;
use nannou::{geom::pt2, glam::Vec2, Draw};

use crate::Settings;

#[derive(Debug, Clone, Copy)]
pub struct Ellipse {
    center: Vec2,
    properties: Settings,
    pub points: Vec<Vec2>,
}

impl Ellipse {
    pub fn new(x: f32, y: f32, settings: Settings) -> Self {
        let r = settings.radius;
        let edges = settings.edges;

        let points = (0..edges)
            .map(|i| {
                let fract = i as f32 / settings.edges as f32;
                let phase = fract;

                let edge_x = r * (TAU * phase).cos();
                let edge_y = r * (TAU * phase).sin();

                pt2(edge_x, edge_y)
            })
            .collect();

        Ellipse { center: pt2(x, y), points, properties: settings }
    }

    pub fn show(self, draw: &Draw) {
        draw.polygon()
            .x_y(self.center.x, self.center.y)
            .color(self.properties.color)
            .stroke(self.properties.stroke)
            .stroke_weight(self.properties.stroke_width)
            .points(self.points);
    }

    pub fn marble(&mut self, other: Ellipse) {
        self.points.iter_mut().for_each(|point| {
            let c = &other.center;
            let r = other.properties.radius;
            let p = self.center.clone();

            let p = pt2(p.x - c.x, p.y - c.y);
            let m = p.x * p.y + p.y * p.y;
            let root = ((1.0 + (r * r) / (m * m)) as f32).sqrt();

            let p = pt2(p.x * root, p.y * root);
            let p = pt2(p.x + c.x, p.y + c.y);

            point.x = p.x;
            point.y = p.y;
        });
    }
}
