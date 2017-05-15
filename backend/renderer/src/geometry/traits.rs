use geometry::BoundingBox3;

// A trait for entities with surface area property.
pub trait HasSurfaceArea {
  fn surface_area(&self) -> f64;
}

// A trait for entities with volume property.
pub trait HasVolume {
  fn volume(&self) -> f64;
}

// A trait for entities with 3-d bounding box.
pub trait HasBoundingBox3 {
  fn bounding_box3(&self) -> BoundingBox3;
}
