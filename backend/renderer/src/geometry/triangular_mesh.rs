use math::Vector3;
use geometry::{
    Triangle3, BoundingBox3, HasBoundingBox3, HasSurfaceArea,
};
use std::vec::Vec;

/// Definitions and operations for a triangular mesh. The mesh is maintained as
/// a **Doubly-Connected Edge List (DCEL)**:
///
/// - Each vertex has its own position and (optionaly) a normal, with a list of
/// outgoing half-edges. 
/// - A face (triangle) contains pointers to its three vertices and three
/// belonging half-edges (going counter-clockwise), optionally a normal vector.
/// - A half-edge and its twin edge (coincide spatially but of opposite
/// directions) define a spatial edge. Each half-edge points from one vertex to
/// another (one-diretional), and belongs to exactly one face (while its twin
/// edge belongs to the adjacent face).
///
/// For more details please read:
/// 
/// > Computational Geometry: Algorithms and Applications (second edition),
/// Chapter 2.2.
/// > M. de Berg, M. van Kreveld, M. Overmars, O. Schwarzkopf.
///
/// DCELs require population and calculation of derived data upon creation.
/// The constructors are private and therefore, you must and you can only build
/// a triangular mesh via TriangularMeshBuilder.
#[derive(Debug, Clone)]
pub struct TriangularMesh {

}

/// Definitions of vertices of DCEL.
#[derive(Debug, Clone)]
struct Vertex {
  // Position of the vertex.
  pos: Vector3,
  // Normal vector of the vertex. Might be None if the parent mesh does not
  // support normal interpolation.
  normal: Option<Vector3>,
  // A list of indices of half-edges started from this vertex (outgoing).
  edges: Vec<usize>,
}

impl Vertex{
  fn new(pos: Vector3) -> Self {
    Self {
      pos: pos,
      normal: None,
      edges: Vec::new() 
    }
  }

  fn new_with_normal(pos: Vector3, normal: Vector3) -> Self {
    Self {
      pos: pos,
      normal: Some(normal),
      edges: Vec::new()
    }
  }
}

/// Definitions of half-edges of DCEL.
#[derive(Debug, Clone)]
struct Edge {
  // Indices of start and end vertices of this edge.
  start_vertex: usize,
  end_vertex: usize,
  // Index of triangle face this edge belongs to.
  face: usize,
  // Index of its twin edge. If its value is None, this edge has no twin edge
  // and thus on the boundary of a manifold.
  twin_edge: Option<usize>,
  // Index of the next edge, which belongs to the same face as this one, and
  // starts from the end vertex of this edge.
  next_edge: usize,
  // Index of the prev edge, which belongs to the same face as this one, and
  // ends at the start vertex of this edge.
  prev_edge: usize,
}

/// Definitions of triangular faces of DCEL.
#[derive(Debug, Clone)]
struct Face {
  // List of all vertices' indices of this triangle.
  vertices: [usize; 3],
  // List of all half-edges belonging to this triangle.
  // Note: edges in this array might not be sorted in counterclock-wise order.
  // Please rely on edge.nextEdge to traverse the face.
  edges: [usize; 3],
  // Normal vector of this face.
  normal: Option<Vector3>,
}

impl Face {
  fn new(vertices: [usize; 3], edges: [usize; 3]) -> Self {
    Self {
      vertices: vertices,
      edges: edges,
      normal: None,
    }
  }

  fn new_with_normal(vertices: [usize; 3], edges: [usize; 3], normal: Vector3)
      -> Self {
    Self {
      vertices: vertices,
      edges: edges,
      normal: Some(normal),
    }
  }
}
