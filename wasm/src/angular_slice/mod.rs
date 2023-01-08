use crate::canvas_point::CanvasPoint;

pub struct AngularSlice {
  start: f64,
  end: f64,
}

impl AngularSlice {
  fn new(start: f64, end: f64) -> Self {
    Self { start, end }
  }

  fn from_offset(start: f64, offset: f64) -> Self {
    Self {
      start,
      end: start + offset,
    }
  }

  fn from_points(
    point_one: CanvasPoint,
    point_two: CanvasPoint,
    control: CanvasPoint,
  ) -> Self {
    Self {
      start: control.angle_to(point_one),
      end: control.angle_to(point_two),
    }
  }
}
