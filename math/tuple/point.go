package tuple

type Point interface {
	FourTuple

	SubPoint(Point) Vector
	SubVector(Vector) Point
}

func NewPoint(x, y, z float64) Point {
	return &point{
		tuple: tuple{
			x: x,
			y: y,
			z: z,
		},
	}
}

// HACK: please make sure that the `point` struct always has the exact
// same memory layout as the `tuple` struct, because there are some
// unsafe pointer operations here which depends on that fact.
//
// HACK: although the `Point` interface allows for either mutable or
// immutable points (all methods return a value), this point is
// implemented with pointer receivers of all methods.  it mutates
// itself and then returns its own pointer in each operation.  this
// should reduce memory allocations.
type point struct {
	tuple
}

func (v point) W() float64 {
	// HACK: in the context of this ray tracer, a point is always
	// four-dimensional and always has a fourth component called w
	// which is always 1.  Therefore we can get away with actually
	// only storing three numbers in the point.
	return 1.0
}

func (p *point) SubPoint(other Point) Vector {
	return (*vector)(p.sub(other).ToUnsafePointer())
}

func (p *point) SubVector(other Vector) Point {
	return (*point)(p.sub(other).ToUnsafePointer())
}
