use dotrix::egui::*;
use crate::ui_progress_bar::widgets::Widget;
use crate::ui_progress_bar::native::Ui;

/// A simple progress bar.
pub struct ProgressBar {
    start: f32,
    progress: f32,
    desired_width: Option<f32>,
    animate: bool,
}

impl ProgressBar {
    /// Progress in the `[0, 1]` range, where `1` means "completed".
    pub fn new(start:f32, progress: f32) -> Self {
        Self {
            start: start.clamp(0.0, 1.0),
            progress: progress.clamp(0.0, 1.0),
            desired_width: None,
            animate: false,
        }
    }

    /// The desired width of the bar. Will use all horizontal space if not set.
    pub fn desired_width(mut self, desired_width: f32) -> Self {
        self.desired_width = Some(desired_width);
        self
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let ProgressBar {
            start,
            progress,
            desired_width,
            animate,
        } = self;

        let animate = animate && progress < 1.0;

        if animate {
            ui.ctx().request_repaint();
        }

        let desired_width =
            desired_width.unwrap_or_else(|| ui.available_size_before_wrap().x.at_least(96.0));
        let height = ui.spacing().interact_size.y;
        let (outer_rect, response) =
            ui.allocate_exact_size(vec2(desired_width, height), Sense::hover());

        if ui.is_rect_visible(response.rect) {
            let visuals = ui.style().visuals.clone();
            let corner_radius = 0.0;
            ui.painter().rect(
                outer_rect,
                corner_radius,
                visuals.extreme_bg_color,
                Stroke::none(),
            );
            let inner_rect = Rect::from_min_size(
                outer_rect.min + vec2(
                    outer_rect.width() * start,
                    0.0
                ),
                vec2(
                    outer_rect.width() * (progress - start).clamp(0.0, 1.0),
                    outer_rect.height(),
                ),
            );

            let (dark, bright) = (0.7, 1.0);
            let color_factor = if animate {
                lerp(dark..=bright, ui.input().time.cos().abs())
            } else {
                bright
            };

            if start < progress {
                ui.painter().rect(
                    inner_rect,
                    corner_radius,
                    Color32::from(Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
                    Stroke::none(),
                );
            }

            if animate {
                let n_points = 20;
                let start_angle = ui.input().time as f64 * 360f64.to_radians();
                let end_angle = start_angle + 240f64.to_radians() * ui.input().time.sin();
                let circle_radius = corner_radius - 2.0;
                let points: Vec<Pos2> = (0..n_points)
                    .map(|i| {
                        let angle = lerp(start_angle..=end_angle, i as f64 / n_points as f64);
                        let (sin, cos) = angle.sin_cos();
                        inner_rect.right_center()
                            + circle_radius * vec2(cos as f32, sin as f32)
                            + vec2(-corner_radius, 0.0)
                    })
                    .collect();
                ui.painter().add(Shape::line(
                    points,
                    Stroke::new(2.0, visuals.faint_bg_color),
                ));
            }
        }
        response
    }
}
