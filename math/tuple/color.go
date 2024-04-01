package tuple

import (
	"fmt"
	"io"
	"math"

	"git.sr.ht/~wgn/ray-tracer-challenge/math/number"
	"git.sr.ht/~wgn/ray-tracer-challenge/world/canvas/properties"
)

const (
	MinColor = float64(0.0)
	MaxColor = float64(255.0)
)

var (
	limit  = number.Interval{Min: 0.0, Max: 1.0}
	output = number.Interval{Min: MinColor, Max: MaxColor}
)

type Color interface {
	ThreeTuple
	properties.Drawable

	Red() float64
	Green() float64
	Blue() float64

	AddColor(Color) Color
	SubColor(Color) Color
	MulColor(Color) Color

	Scale(float64) Color
}

func NewColor(red, green, blue float64) Color {
	return &color{
		tuple: tuple{
			x: red,
			y: green,
			z: blue,
		},
	}
}

// convenience function that returns a white `Color`:
func White() Color {
	return NewColor(1.0, 1.0, 1.0)
}

// convenience function that returns a black `Color`:
func Black() Color {
	return NewColor(0.0, 0.0, 0.0)
}

// HACK: please make sure that the `color` struct always has the exact
// same memory layout as the `tuple` struct, because there are some
// unsafe pointer operations here which depends on that fact.
//
// HACK: although the `Color` interface allows for either mutable or
// immutable colors (all relevant methods return a value), this color
// is implemented with pointer receivers of all methods.  it mutates
// itself and then returns its own pointer in each operation.  this
// should reduce memory allocations.
type color struct {
	tuple
}

func (c color) X() float64 {
	return c.x
}

func (c color) Red() float64 {
	return c.X()
}

func (c color) Y() float64 {
	return c.y
}

func (c color) Green() float64 {
	return c.Y()
}

func (c color) Z() float64 {
	return c.z
}

func (c color) Blue() float64 {
	return c.Z()
}

func (c color) W() float64 {
	return 0.0
}

func (c *color) AddColor(other Color) Color {
	return (*color)(c.add(other).ToUnsafePointer())
}

func (c *color) SubColor(other Color) Color {
	return (*color)(c.sub(other).ToUnsafePointer())
}

func (c *color) MulColor(other Color) Color {
	c.x *= other.X()
	c.y *= other.Y()
	c.z *= other.Z()

	return c
}

func (c *color) Scale(scalar float64) Color {
	c.x *= scalar
	c.y *= scalar
	c.z *= scalar

	return c
}

func (c color) ToPPM(w io.Writer) {
	io.WriteString(w, fmt.Sprintf("%d %d %d",
		int64(math.Round(number.ChangeInterval(
			limit.Clamp(c.Red()),
			limit,
			output,
		))),
		int64(math.Round(number.ChangeInterval(
			limit.Clamp(c.Green()),
			limit,
			output,
		))),
		int64(math.Round(number.ChangeInterval(
			limit.Clamp(c.Blue()),
			limit,
			output,
		)))))
}
