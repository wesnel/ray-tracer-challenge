package canvas

import (
	"git.sr.ht/~wgn/ray-tracer-challenge/world/canvas/properties"
)

type Option func(*canvas)

type (
	FillFunc   func(width, height uint64) properties.Drawable
	HeaderFunc func(c Canvas) string
)

func WithFillFunc(f FillFunc) Option {
	return func(c *canvas) {
		for i := range c.contents {
			c.contents[i] = f(uint64(i)%c.width, uint64(i)/c.width)
		}
	}
}

func WithHeader(f HeaderFunc) Option {
	return func(c *canvas) {
		c.header = f
	}
}

func WithSeparator(s string) Option {
	return func(c *canvas) {
		c.separator = s
	}
}
