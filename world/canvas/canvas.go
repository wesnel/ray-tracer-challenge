package canvas

import (
	"fmt"
	"io"

	"git.sr.ht/~wgn/ray-tracer-challenge/math/tuple"
	"git.sr.ht/~wgn/ray-tracer-challenge/world/canvas/properties"
)

var defaultOptions = []Option{
	WithFillFunc(func(_, _ uint64) properties.Drawable {
		return tuple.Black()
	}),
	WithHeader(func(c Canvas) string {
		return fmt.Sprintf("P3\n%d %d\n%d\n",
			c.Width(),
			c.Height(),
			int64(tuple.MaxColor))
	}),
	WithSeparator("\n"),
}

type Canvas interface {
	properties.Drawable

	Width() uint64
	Height() uint64
	Contents() []properties.Drawable

	Get(uint64, uint64) properties.Drawable
	Set(uint64, uint64, properties.Drawable) Canvas
}

func New(width, height uint64, opts ...Option) Canvas {
	c := &canvas{
		width:    width,
		height:   height,
		contents: make([]properties.Drawable, height*width),
	}

	for _, opt := range defaultOptions {
		opt(c)
	}

	for _, opt := range opts {
		opt(c)
	}

	return c
}

type canvas struct {
	width     uint64
	height    uint64
	header    HeaderFunc
	separator string
	contents  []properties.Drawable
}

func (c canvas) Width() uint64 {
	return c.width
}

func (c canvas) Height() uint64 {
	return c.height
}

func (c canvas) Contents() []properties.Drawable {
	return c.contents
}

func (c *canvas) Get(x, y uint64) properties.Drawable {
	return c.contents[c.index(x, y)]
}

func (c *canvas) Set(x, y uint64, item properties.Drawable) Canvas {
	c.contents[c.index(x, y)] = item

	return c
}

func (c canvas) index(x, y uint64) uint64 {
	return x + y*c.width
}

func (c canvas) ToPPM(w io.Writer) {
	io.WriteString(w, c.header(&c))

	for _, item := range c.contents {
		item.ToPPM(w)
		io.WriteString(w, c.separator)
	}
}
