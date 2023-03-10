pub mod contiguous_shape_builder;
mod contiguous_shape_from_contiguous_shape_builder;
mod contiguous_shape_from_stroke_kind;

use web_sys::CanvasRenderingContext2d;

use crate::{
  canvas_point::CanvasPoint,
  enums::{intersect_type::IntersectType, stroke_kind::StrokeKind},
  js_panic,
  shape_config::ShapeConfig,
  shape_segment::ShapeSegment,
};

#[derive(Debug, Default)]
pub struct ContiguousShape {
  pub start: CanvasPoint,
  pub closed_shape: bool,
  pub filled_shape: bool,
  pub segments: Vec<ShapeSegment>,
  pub config: ShapeConfig,
  pub intersect_type: IntersectType,
}

impl ContiguousShape {
  pub fn draw(&self, context: &CanvasRenderingContext2d) {
    context.begin_path();
    context.set_stroke_style(&self.config.style.to_string().into());
    context.set_line_width(self.config.width.into());
    context.set_line_cap(&self.config.cap.to_string());
    context.move_to(self.start.x, self.start.y);
    for segment in self.segments.iter() {
      match segment.stroke_kind {
        StrokeKind::Line => {
          context.line_to(segment.coordinates.x, segment.coordinates.y);
        }
        StrokeKind::Square(corner_one, corner_two) => {
          let x_diff = (corner_two.x - corner_one.x) / 2.0;
          let y_diff = (corner_two.y - corner_one.y) / 2.0;
          let center_x = (corner_one.x + corner_two.x) / 2.0;
          let center_y = (corner_one.y + corner_two.y) / 2.0;
          context.move_to(corner_one.x, corner_one.y);
          context.line_to(center_x - y_diff, center_y + x_diff);
          context.line_to(corner_two.x, corner_two.y);
          context.line_to(center_x + y_diff, center_y - x_diff);
          context.close_path();
        }
        StrokeKind::SquareVector(corner_one, angle, radius) => {
          let corner_two: CanvasPoint = (
            corner_one.x + (angle.cos() * radius),
            corner_one.y + (angle.sin() * radius),
          )
            .into();
          let x_diff = (corner_two.x - corner_one.x) / 2.0;
          let y_diff = (corner_two.y - corner_one.y) / 2.0;
          let center_x = (corner_one.x + corner_two.x) / 2.0;
          let center_y = (corner_one.y + corner_two.y) / 2.0;
          context.move_to(corner_one.x, corner_one.y);
          context.line_to(center_x - y_diff, center_y + x_diff);
          context.line_to(corner_two.x, corner_two.y);
          context.line_to(center_x + y_diff, center_y - x_diff);
          context.close_path();
        }
        StrokeKind::CircularArc(center, radius, slice) => {
          context.begin_path();
          let Ok(_) = context.arc(
            center.x,
            center.y,
            radius,
            slice.start,
            slice.end
          ) else {
            js_panic!("Invalid parameters passed to arc function");
          };
        }
        StrokeKind::ControlledArc(control, radius) => {
          let Ok(_) = context.arc_to(
            control.x,
            control.y,
            segment.coordinates.x,
            segment.coordinates.y,
            radius,
          ) else {
            js_panic!("Invalid parameters passed to arc_to function");
          };
        }
        StrokeKind::BezierCurve(control_one, control_two) => {
          context.bezier_curve_to(
            control_one.x,
            control_one.y,
            control_two.x,
            control_two.y,
            segment.coordinates.x,
            segment.coordinates.y,
          );
        }
        StrokeKind::Ellipse(center, radius_x, radius_y, rotation, slice) => {
          context.begin_path();
          let Ok(_) = context.ellipse(
            center.x,
            center.y,
            radius_x,
            radius_y,
            rotation,
            slice.start,
            slice.end,
          ) else {
            js_panic!("Invalid parameters passed to ellipse function");
          };
        }
      }
    }
    if self.closed_shape {
      context.close_path();
    };
    if self.filled_shape {
      context.set_fill_style(&self.config.fill.to_string().into());
      context.fill();
    }
    context.stroke();
  }
}
