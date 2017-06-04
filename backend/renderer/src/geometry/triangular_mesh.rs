use math::Vector3;
use geometry::{
    Triangle3, BoundingBox3, HasBoundingBox3, HasSurfaceArea,
};
use common::constants;
use common::utils;
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
pub struct Vertex {
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
pub struct Edge {
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
pub struct Face {
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

/// Definition of different modes of normal vector calculation on vertices.
pub enum VertexNormalMode {
  /// User specifies normal vector for each vertex.
  USER_SPECIFIED,
  /// Normal of a vertex is calculated by averaging normals of all its adjacent
  /// faces. If face normals are provided by users, we use the provided value.
  /// Otherwise We use the flat normal (the unit normal vector orthoganal to
  /// the face).
  ///
  /// TODO: There is actually a huge problem with averaged vertex normal, 
  /// that the averaged normal n might be below some triangle, in other words:
  /// for some n_i that is a flat surface adjacent to this vertex, n * n_i < 0.
  /// Using such averaged normal might cause Phong-interpolated normal on such
  /// face be completely on the wrong direction. Forcing the normal to be
  /// negated will then cause discontinuity of the normal field across the
  /// mesh.
  AVERAGED
}

/// Definition of different modes of normal vector calculation on faces.
pub enum FaceNormalMode {
  /// User specifies normal vector for each face.
  USER_SPECIFIED,
  /// Use the unit normal vector orthoganal to the face.
  FLAT,
  /// Linearly interpolate normal vector from normals of all its vertices.
  /// More specifically, if a point p on a triangle can be represented as:
  ///
  /// > p = a * v1 + b * v2 + c * v3,
  ///
  /// where vi's are the coordinates of the three vertices of the face and
  /// a, b, c are positive real numbers satisfying a + b + c = 1, then the
  /// normal of p, denoted as N(p), is calculated as:
  ///
  /// > N(p) = a * N(v1) + b * N(v2) + c * N(v3).
  ///
  /// **Note**: if vertex mode is not USER_SPECIFIED, then we first need to
  /// calculate vertex normal by averaging flat normals of faces, then use the
  /// averaged vertex normal to do Phong interpolation.
  PHONG
}

/// A descriptor for any point on the triangular mesh, represented by a face
/// index and parametric representation of the point. More specifically, if the
/// face f is formed vertex v0, v1 and v2 (in this order), the representation of
/// a point p = (f, a, b, c) on this face represents the point:
///
/// > p = a * v0 + b * v1 + c * v2
///
/// Note that 0 <= a, b, c <= 1 and a + b + c = 1 must be satisfied.
///
/// Users of this class should never care about the order and value of the
/// parameters as it's closely related to the representation of its belonging
/// triangular mesh, whose order of vertices in a face is also an internal data.
#[derive(Debug, Copy, Clone)]
pub struct MeshPoint {
  face_id: usize,
  params: Vector3,
}

impl MeshPoint {
  pub fn new(face_id: usize, params: Vector3) -> Self {
    assert!(params[0] >= 0.0 && params[0] <= 1.0
        &&  params[1] >= 0.0 && params[1] <= 1.0
        &&  params[2] >= 0.0 && params[2] <= 1.0
        &&  utils::equal_with_bound(params[0] + params[1] + params[2], 1.0),
        "Params must be positive real numbers summing up exactly to 1!");
    Self {
      face_id: face_id,
      params: params,
    }
  }
}
