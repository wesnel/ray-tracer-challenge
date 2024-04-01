package properties

import (
	"io"
)

type Drawable interface {
	ToPPM(io.Writer)
}
