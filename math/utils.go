package math

import (
	"math"
)

// saving a difficult regexp here for reuse:
const FloatFormat = `(-?\d+\.?\d*)`

// small number for testing float equality:
var Epsilon = 0.00001

// approximately tests the equality of two floating point numbers by
// checking if their absolute difference is within a threshold:
func Equals(epsilon float64) func(float64, float64) bool {
	return func(expected, got float64) bool {
		return math.Abs(expected-got) < epsilon
	}
}
