package tuple

import (
	"unsafe"
)

type FourTuple interface {
	ThreeTuple

	W() float64

	add(ThreeTuple) ThreeTuple
	sub(ThreeTuple) ThreeTuple
}

type ThreeTuple interface {
	X() float64
	Y() float64
	Z() float64

	ToUnsafePointer() unsafe.Pointer
}

type tuple struct {
	x float64
	y float64
	z float64
}

func (t tuple) X() float64 {
	return t.x
}

func (t tuple) Y() float64 {
	return t.y
}

func (t tuple) Z() float64 {
	return t.z
}

func (t *tuple) ToUnsafePointer() unsafe.Pointer {
	return unsafe.Pointer(t)
}

func (t *tuple) add(other ThreeTuple) ThreeTuple {
	t.x += other.X()
	t.y += other.Y()
	t.z += other.Z()

	return t
}

func (t *tuple) sub(other ThreeTuple) ThreeTuple {
	t.x -= other.X()
	t.y -= other.Y()
	t.z -= other.Z()

	return t
}
