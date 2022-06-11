use dotrix::egui::*;
use dotrix::egui::emath::{Pos2, Vec2};
use dotrix::egui::epaint::{PathShape, Stroke};

use crate::ui_clock::widgets::Widget;
use crate::ui_clock::native::Ui;

use std::f32::consts::PI;

pub struct Clock {
    elapsed: f32,
    current_rewind: f32,
    max_rewind: f32,
    size: f32,
}

impl Clock {
    pub fn new(elapsed:f32, current_rewind: f32, max_rewind: f32) -> Self {
        Self {
            elapsed: elapsed,
            current_rewind: current_rewind,
            max_rewind: max_rewind,
            size: 64.0,
        }
    }
}

impl Widget for Clock {
    fn ui(self, ui: &mut Ui) -> Response {
        let Clock {
            elapsed,
            current_rewind,
            max_rewind,
            size,
        } = self;

        let full_clock = elapsed >= 2.0*PI;
        let max_rewind_full = max_rewind >= 2.0*PI;

        let offset = PI/2.0;

        let mut angles = Vec::new();
        let mut lengths = Vec::new();
        let mut colors = Vec::new();

        if full_clock {
            if !max_rewind_full {
                angles.push(elapsed);
                lengths.push(2.0*PI-max_rewind);
                colors.push(Color32::from_gray(64));
            }
        } else {
            let inactive_end_angle = (0.0 as f32).min(elapsed-max_rewind);

            angles.push(inactive_end_angle);
            lengths.push(elapsed - 2.0*PI - inactive_end_angle);
            colors.push(Color32::from_gray(8));

            if elapsed-max_rewind > 0.0 {
                angles.push(0.0);
                lengths.push(elapsed-max_rewind);
                colors.push(Color32::from_gray(64));
            }
        }

        if (max_rewind-current_rewind) > 0.0 {
            if !full_clock & (elapsed-max_rewind < 0.0) {
                angles.push(0.0);
                lengths.push(elapsed-max_rewind);
                colors.push(Color32::from_rgb(29,122,29));

                angles.push(0.0);
                lengths.push(elapsed-current_rewind);
                colors.push(Color32::from_rgb(50,205,50));
            } else {
                angles.push(elapsed-max_rewind);
                lengths.push(max_rewind-current_rewind);
                colors.push(Color32::from_rgb(50,205,50));
            }
        }

        if current_rewind > 0.0 {
            angles.push(elapsed);
            lengths.push(-current_rewind);
            colors.push(Color32::from_rgb(153,153,255));
        }

        let (_, response) =
            ui.allocate_exact_size(vec2(size, size), Sense::hover());

        if ui.is_rect_visible(response.rect) {
            for i in 0..lengths.len(){
                if lengths[i].abs() > PI {
                    ui.painter().add(arc_shape(
                        response.rect.center(),
                        response.rect.width()/2.0,
                        colors[i],
                        angles[i]-offset,
                        lengths[i]/2.0,
                    ));
                    ui.painter().add(arc_shape(
                        response.rect.center(),
                        response.rect.width()/2.0,
                        colors[i],
                        angles[i]+lengths[i]/2.0-offset,
                        lengths[i]/2.0,
                    ));
                } else {
                    ui.painter().add(arc_shape(
                        response.rect.center(),
                        response.rect.width()/2.0,
                        colors[i],
                        angles[i]-offset,
                        lengths[i],
                    ));
                }
            }

            ui.painter().arrow(
                response.rect.center(),
                Vec2::new(
                    response.rect.width()/2.2 * (elapsed-current_rewind-offset).cos(),
                    response.rect.width()/2.2 * (elapsed-current_rewind-offset).sin()
                ),
                Stroke::new(2.0, Color32::from_gray(205))
            )
        }
        response
    }
}

fn arc_shape(
    center: Pos2,
    radius: f32,
    fill: Color32,
    mut angle_offset: f32,
    mut arc_length: f32,
) -> PathShape {

    angle_offset %= 2.0*PI;
    if arc_length > PI {
        arc_length = PI;
    }

    if arc_length < 0.0 {
        arc_length = -arc_length;
        angle_offset -= arc_length;
    }

    let d_a = PI/36.0;

    let mut points = vec!(center);

    let mut angle = angle_offset;

    while angle < arc_length + angle_offset {
        points.push(
            center + Vec2::new(angle.cos()*radius, angle.sin()*radius)
        );

        angle += d_a;
    }

    angle = arc_length + angle_offset;
    points.push(
        center + Vec2::new(angle.cos()*radius, angle.sin()*radius)
    );

    PathShape{
        points,
        closed: true,
        fill,
        stroke: Stroke::new(2.0, fill),
    }
}
