// The error bound for comparing floating point numericals, i.e. numericals within this
// error bound are considered equal.
pub const EPSILON: f64 = 1e-6;

// A finer error bound for comparing values with zeros. i.e. numericals with absolute value
// less than this constant will be considered zero.
pub const EPSILON_TINY: f64 = 1e-9;

// Represents invalid id. Note that ids are mostly represented as unsigned ints and longs,
// therefore this value should usually be converted to very large numbers (2^32-1 or 2^64-1),
// which is rarely reached and thus serve as an invalid id.
pub const INVALID_ID: i64 = -1;

// A sufficiently large number representing (positive) infinity.
pub const INFINITY: f64 = 1e20;
