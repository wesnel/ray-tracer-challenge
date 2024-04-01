package tuple

import (
	"unsafe"
)

type Point interface {
	FourTuple

	SubPoint(Point) Vector
	SubVector(Vector) Point
}

func NewPoint(x, y, z float64) Point {
	return &point{
		x: x,
		y: y,
		z: z,
	}
}

// HACK: Please make sure that the `point` struct always has the exact
// same memory layout as `vector` struct, because there are some
// unsafe pointer operations here which depends on that fact.
//
// HACK: although the `Point` interface allows for either mutable or
// immutable points (all methods return a value), this point is
// implemented with pointer receivers of all methods.  it mutates
// itself and then returns its own pointer in each operation.  this
// should reduce memory allocations.
type point struct {
	x float64
	y float64
	z float64
}

func (v point) X() float64 {
	return v.x
}

func (v point) Y() float64 {
	return v.y
}

func (v point) Z() float64 {
	return v.z
}

func (v point) W() float64 {
	// HACK: in the context of this ray tracer, a point is always
	// four-dimensional and always has a fourth component called w
	// which is always 1.  Therefore we can get away with actually
	// only storing three numbers in the point.
	return 1.0
}

func (p *point) SubPoint(other Point) Vector {
	p.x -= other.X()
	p.y -= other.Y()
	p.z -= other.Z()

	// HACK: Please make sure that the `point` struct always has
	//       the exact same memory layout as `vector` struct:
	return (*vector)(unsafe.Pointer(p))
}

func (p *point) SubVector(other Vector) Point {
	p.x -= other.X()
	p.y -= other.Y()
	p.z -= other.Z()

	return p
}
