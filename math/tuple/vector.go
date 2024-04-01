package tuple

import (
	"math"
)

type Vector interface {
	FourTuple

	AddVector(Vector) Vector
	SubVector(Vector) Vector
	CrossProduct(Vector) Vector

	Scale(float64) Vector
	Div(float64) Vector

	Dot(Vector) float64
	Magnitude() float64

	Normalize() Vector
}

func NewVector(x, y, z float64) Vector {
	return &vector{
		x: x,
		y: y,
		z: z,
	}
}

// HACK: Please make sure that the `vector` struct always has the
// exact same memory layout as `point` struct, because there are some
// unsafe pointer operations here which depends on that fact.
//
// HACK: although the `Vector` interface allows for either mutable or
// immutable vectors (all methods return a value), this vector is
// implemented with pointer receivers of all methods.  it mutates
// itself and then returns its own pointer in each operation.  this
// should reduce memory allocations.
type vector struct {
	x float64
	y float64
	z float64
}

func (v vector) X() float64 {
	return v.x
}

func (v vector) Y() float64 {
	return v.y
}

func (v vector) Z() float64 {
	return v.z
}

func (v vector) W() float64 {
	// HACK: in the context of this ray tracer, a vector is always
	// four-dimensional and always has a fourth component called w
	// which is always 1.  Therefore we can get away with actually
	// only storing three numbers in the vector.
	return 0.0
}

func (v *vector) AddVector(other Vector) Vector {
	v.x += other.X()
	v.y += other.Y()
	v.z += other.Z()

	return v
}

func (v *vector) SubVector(other Vector) Vector {
	v.x -= other.X()
	v.y -= other.Y()
	v.z -= other.Z()

	return v
}

func (v vector) CrossProduct(other Vector) Vector {
	return NewVector(
		v.y*other.Z()-v.z*other.Y(),
		v.z*other.X()-v.x*other.Z(),
		v.x*other.Y()-v.y*other.X(),
	)
}

func (v *vector) Scale(scalar float64) Vector {
	v.x *= scalar
	v.y *= scalar
	v.z *= scalar

	return v
}

func (v *vector) Div(scalar float64) Vector {
	v.x /= scalar
	v.y /= scalar
	v.z /= scalar

	return v
}

func (v vector) Dot(other Vector) float64 {
	return v.x*other.X() + v.y*other.Y() + v.z*other.Z()
}

func (v vector) Magnitude() float64 {
	return math.Sqrt(v.Dot(&v))
}

func (v *vector) Normalize() Vector {
	return v.Div(v.Magnitude())
}
