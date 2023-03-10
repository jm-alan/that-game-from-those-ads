use crate::canvas_point::CanvasPoint;

#[derive(Debug, Default, Clone, Copy)]
pub struct AngularSlice {
  pub start: f64,
  pub end: f64,
}

impl AngularSlice {
  pub fn new(start: f64, end: f64) -> Self {
    Self { start, end }
  }

  pub fn from_offset(start: f64, offset: f64) -> Self {
    Self {
      start,
      end: start + offset,
    }
  }

  pub fn from_points(
    point_one: CanvasPoint,
    point_two: CanvasPoint,
    control: CanvasPoint,
  ) -> Self {
    Self {
      start: control.angle_to(point_one),
      end: control.angle_to(point_two),
    }
  }

  pub fn circle() -> Self {
    Self {
      start: 0.0,
      end: std::f64::consts::PI * 2.0,
    }
  }

  pub fn semicircle() -> Self {
    Self {
      start: 0.0,
      end: std::f64::consts::PI,
    }
  }

  pub fn semicircle_from(start: f64) -> Self {
    Self {
      start,
      end: start + std::f64::consts::PI,
    }
  }
}
