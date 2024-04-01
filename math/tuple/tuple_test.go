package tuple_test

import (
	"context"
	"embed"
	"fmt"
	"testing"

	"github.com/cucumber/godog"

	"git.sr.ht/~wgn/ray-tracer-challenge/math"
	"git.sr.ht/~wgn/ray-tracer-challenge/math/tuple"
)

// embed the cucumber feature files.  this is probably a little bit
// more safe/portable/fast than just providing the folder name to
// godog:
//
//go:embed features/*.feature
var features embed.FS

// functions to get all the values in a three-tuple:
var threeTupleGetters = map[string]func(tuple.ThreeTuple) float64{
	"x": func(t tuple.ThreeTuple) float64 { return t.X() },
	"y": func(t tuple.ThreeTuple) float64 { return t.Y() },
	"z": func(t tuple.ThreeTuple) float64 { return t.Z() },
}

// functions to get all the values in a four-tuple:
var fourTupleGetters = map[string]func(tuple.FourTuple) float64{
	"x": func(t tuple.FourTuple) float64 { return t.X() },
	"y": func(t tuple.FourTuple) float64 { return t.Y() },
	"z": func(t tuple.FourTuple) float64 { return t.Z() },
	"w": func(t tuple.FourTuple) float64 { return t.W() },
}

// functions to get all the channels from a color:
var channels = map[string]func(tuple.Color) float64{
	"red":   func(c tuple.Color) float64 { return c.Red() },
	"green": func(c tuple.Color) float64 { return c.Green() },
	"blue":  func(c tuple.Color) float64 { return c.Blue() },
}

// initialization functions for the test scenarios:
var scenarios = []func(*godog.ScenarioContext){
	tuples,
	vectors,
	points,
	colors,
}

func tuples(sc *godog.ScenarioContext) {
	// tuple field validation:
	for field, getter := range fourTupleGetters {
		sc.Step(
			fmt.Sprintf(`^(\w+)\.%s = %s$`,
				field,
				math.FloatFormat),
			tupleHasValue(field, getter))
	}

	// point or vector equality with tuple:
	sc.Step(
		fmt.Sprintf(`^(\w+) = tuple\(%s, %s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		tupleEqualsTuple)
}

func vectors(sc *godog.ScenarioContext) {
	// vector creation:
	sc.Given(
		fmt.Sprintf(`^(\w+) <- vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat,
		),
		givenVector)

	// vector normalization:
	sc.Step(
		`^(\w+) <- normalize\((\w+)\)$`,
		givenNormalizedVector)

	// vector addition:
	sc.Step(
		fmt.Sprintf(`^(\w+) \+ (\w+) = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		addingVectorToVectorEqualsVector)

	// vector subtraction:
	sc.Step(
		fmt.Sprintf(`^(v\w*|zero) - (v\w*|zero) = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		subtractingVectorFromVectorEqualsVector)

	// negating a vector:
	sc.Step(
		fmt.Sprintf(`^-(\w+) = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		negatingVector)

	// multiplying vector by scalar:
	sc.Step(
		fmt.Sprintf(`^(\w+) \* %s = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		multiplyingVectorByScalar)

	// dividing vector by scalar:
	sc.Step(
		fmt.Sprintf(`^(\w+) / %s = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		dividingVectorByScalar)

	// magnitude of vector:
	sc.Step(
		fmt.Sprintf(`^magnitude\((\w+)\) = %s$`,
			math.FloatFormat),
		vectorMagnitude)

	// normalized vector:
	sc.Step(
		fmt.Sprintf(`^normalize\((\w+)\) = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		normalizedVector)

	// dot product of two vectors:
	sc.Step(
		fmt.Sprintf(`^dot\((\w+), (\w+)\) = %s$`,
			math.FloatFormat),
		vectorDotProduct)

	// cross product of two vectors:
	sc.Step(
		fmt.Sprintf(`^cross\((\w+), (\w+)\) = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		vectorCrossProduct)
}

func points(sc *godog.ScenarioContext) {
	// point creation:
	sc.Given(
		fmt.Sprintf(`^(\w+) <- point\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat,
		),
		givenPoint)

	// point subtraction:
	sc.Step(
		fmt.Sprintf(`^(p\w*) - (p\w*) = vector\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		subtractingPointFromPointEqualsVector)

	// subtracting vector from point:
	sc.Step(
		fmt.Sprintf(`^(\w+) - (\w+) = point\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		subtractingVectorFromPointEqualsPoint)
}

func colors(sc *godog.ScenarioContext) {
	// color creation:
	sc.Given(
		fmt.Sprintf(`^(\w+) <- color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		givenColor)

	// color field validation:
	for field, getter := range channels {
		sc.Step(
			fmt.Sprintf(`^(\w+)\.%s = %s$`,
				field,
				math.FloatFormat),
			colorHasValue(field, getter))
	}

	// color addition:
	sc.Step(
		fmt.Sprintf(`^(\w+) \+ (\w+) = color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		addingColorToColorEqualsColor)

	// color subtraction:
	sc.Step(
		fmt.Sprintf(`^(\w+) - (\w+) = color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		subtractingColorFromColorEqualsColor)

	// multiplying color by scalar:
	sc.Step(
		fmt.Sprintf(`^(\w+) \* %s = color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		multiplyingColorByScalar)

	// multiplying colors:
	sc.Step(
		fmt.Sprintf(`^(\w+) \* (c\w*) = color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		multiplyingColorWithColorEqualsColor)
}

type ctxKey string

func givenPoint(
	ctx context.Context,
	name string,
	x,
	y,
	z float64,
) (context.Context, error) {
	return context.WithValue(ctx, ctxKey(name), tuple.NewPoint(x, y, z)), nil
}

func givenVector(
	ctx context.Context,
	name string,
	x,
	y,
	z float64,
) (context.Context, error) {
	return context.WithValue(ctx, ctxKey(name), tuple.NewVector(x, y, z)), nil
}

func givenNormalizedVector(
	ctx context.Context,
	new,
	orig string,
) (context.Context, error) {
	copy, err := newNormalizedVector(ctx, orig)
	if err != nil {
		return ctx, err
	}

	return context.WithValue(ctx, ctxKey(new), copy), nil
}

func givenColor(
	ctx context.Context,
	name string,
	red,
	green,
	blue float64,
) (context.Context, error) {
	return context.WithValue(ctx, ctxKey(name), tuple.NewColor(red, green, blue)), nil
}

func newNormalizedVector(
	ctx context.Context,
	name string,
) (tuple.Vector, error) {
	given, err := getVectorByName(ctx, name)
	if err != nil {
		return nil, err
	}

	// HACK: the wording of these cucumber tests indicate that the
	//       original vector ought to be immutable, but our
	//       normalization implementation mutates the input
	//       vector.  therefore, make a copy.
	return tuple.NewVector(given.X(), given.Y(), given.Z()).Normalize(), nil
}

func getTupleByName(
	ctx context.Context,
	name string,
) (tuple.FourTuple, error) {
	got, ok := ctx.Value(ctxKey(name)).(tuple.FourTuple)
	if !ok {
		return got, fmt.Errorf("invalid tuple variable name %s",
			name)
	}

	return got, nil
}

func getPointByName(
	ctx context.Context,
	name string,
) (tuple.Point, error) {
	got, ok := ctx.Value(ctxKey(name)).(tuple.Point)
	if !ok {
		return got, fmt.Errorf("invalid point variable name %s",
			name)
	}

	return got, nil
}

func getVectorByName(
	ctx context.Context,
	name string,
) (tuple.Vector, error) {
	got, ok := ctx.Value(ctxKey(name)).(tuple.Vector)
	if !ok {
		return got, fmt.Errorf("invalid vector variable name %s",
			name)
	}

	return got, nil
}

func getColorByName(
	ctx context.Context,
	name string,
) (tuple.Color, error) {
	got, ok := ctx.Value(ctxKey(name)).(tuple.Color)
	if !ok {
		return got, fmt.Errorf("invalid color variable name %s",
			name)
	}

	return got, nil
}

func threeTupleEquality(
	name string,
	expected,
	got tuple.ThreeTuple,
) error {
	for field, getter := range threeTupleGetters {
		if err := compareValues(
			name,
			field,
			getter(expected),
			getter(got),
		); err != nil {
			return err
		}
	}

	return nil
}

func fourTupleEquality(
	name string,
	expected,
	got tuple.FourTuple,
) error {
	for field, getter := range fourTupleGetters {
		if err := compareValues(
			name,
			field,
			getter(expected),
			getter(got),
		); err != nil {
			return err
		}
	}

	return nil
}

func compareValues(
	name,
	field string,
	expected,
	got float64,
) error {
	if !math.Equals(math.Epsilon)(expected, got) {
		return fmt.Errorf("for %s of tuple %s: expected %v but got %v",
			field,
			name,
			expected,
			got)
	}

	return nil
}

func tupleHasValue(
	field string,
	getter func(tuple.FourTuple) float64,
) func(
	ctx context.Context,
	name string,
	expected float64,
) error {
	return func(
		ctx context.Context,
		name string,
		expected float64,
	) error {
		got, err := getTupleByName(ctx, name)
		if err != nil {
			return err
		}

		return compareValues(
			name,
			field,
			expected,
			getter(got),
		)
	}
}

func colorHasValue(
	field string,
	getter func(tuple.Color) float64,
) func(
	ctx context.Context,
	name string,
	expected float64,
) error {
	return func(
		ctx context.Context,
		name string,
		expected float64,
	) error {
		got, err := getColorByName(ctx, name)
		if err != nil {
			return err
		}

		return compareValues(
			name,
			field,
			expected,
			getter(got),
		)
	}
}

func tupleEqualsTuple(
	ctx context.Context,
	name string,
	x,
	y,
	z,
	w float64,
) error {
	got, err := getTupleByName(ctx, name)
	if err != nil {
		return err
	}

	switch w {
	case 0.0:
		return fourTupleEquality(name, tuple.NewVector(x, y, z), got)
	case 1.0:
		return fourTupleEquality(name, tuple.NewPoint(x, y, z), got)
	default:
		return fmt.Errorf("invalid w value %v", w)
	}
}

func testBinaryOperationReturningFourTuple[
	A,
	B any,
	C tuple.FourTuple,
](
	getLeft func() (A, error),
	getRight func() (B, error),
	operation func(A, B) C,
	description string,
	expected C,
) error {
	left, err := getLeft()
	if err != nil {
		return err
	}

	right, err := getRight()
	if err != nil {
		return err
	}

	got := operation(left, right)

	return fourTupleEquality(
		description,
		expected,
		got,
	)
}

func testBinaryOperationReturningThreeTuple[
	A,
	B any,
	C tuple.ThreeTuple,
](
	getLeft func() (A, error),
	getRight func() (B, error),
	operation func(A, B) C,
	description string,
	expected C,
) error {
	left, err := getLeft()
	if err != nil {
		return err
	}

	right, err := getRight()
	if err != nil {
		return err
	}

	got := operation(left, right)

	return threeTupleEquality(
		description,
		expected,
		got,
	)
}

func testBinaryOperationReturningScalar[A, B any](
	getLeft func() (A, error),
	getRight func() (B, error),
	operation func(A, B) float64,
	description string,
	expected float64,
) error {
	left, err := getLeft()
	if err != nil {
		return err
	}

	right, err := getRight()
	if err != nil {
		return err
	}

	got := operation(left, right)

	if err := compareValues(
		description,
		"result",
		expected,
		got,
	); err != nil {
		return err
	}

	return nil
}

func addingVectorToVectorEqualsVector(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, leftName)
		},
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, rightName)
		},
		func(left, right tuple.Vector) tuple.Vector {
			return left.AddVector(right)
		},
		fmt.Sprintf("%s + %s", leftName, rightName),
		tuple.NewVector(x, y, z),
	)
}

func addingColorToColorEqualsColor(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningThreeTuple(
		func() (tuple.Color, error) {
			return getColorByName(ctx, leftName)
		},
		func() (tuple.Color, error) {
			return getColorByName(ctx, rightName)
		},
		func(left, right tuple.Color) tuple.Color {
			return left.AddColor(right)
		},
		fmt.Sprintf("%s + %s", leftName, rightName),
		tuple.NewColor(x, y, z),
	)
}

func subtractingPointFromPointEqualsVector(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Point, error) {
			return getPointByName(ctx, leftName)
		},
		func() (tuple.Point, error) {
			return getPointByName(ctx, rightName)
		},
		func(left, right tuple.Point) tuple.Vector {
			return left.SubPoint(right)
		},
		fmt.Sprintf("%s - %s", leftName, rightName),
		tuple.NewVector(x, y, z),
	)
}

func subtractingVectorFromPointEqualsPoint(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Point, error) {
			return getPointByName(ctx, leftName)
		},
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, rightName)
		},
		func(left tuple.Point, right tuple.Vector) tuple.Point {
			return left.SubVector(right)
		},
		fmt.Sprintf("%s - %s", leftName, rightName),
		tuple.NewPoint(x, y, z),
	)
}

func subtractingVectorFromVectorEqualsVector(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, leftName)
		},
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, rightName)
		},
		func(left, right tuple.Vector) tuple.Vector {
			return left.SubVector(right)
		},
		fmt.Sprintf("%s - %s", leftName, rightName),
		tuple.NewVector(x, y, z),
	)
}

func subtractingColorFromColorEqualsColor(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningThreeTuple(
		func() (tuple.Color, error) {
			return getColorByName(ctx, leftName)
		},
		func() (tuple.Color, error) {
			return getColorByName(ctx, rightName)
		},
		func(left, right tuple.Color) tuple.Color {
			return left.SubColor(right)
		},
		fmt.Sprintf("%s - %s", leftName, rightName),
		tuple.NewColor(x, y, z),
	)
}

func multiplyingColorWithColorEqualsColor(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningThreeTuple(
		func() (tuple.Color, error) {
			return getColorByName(ctx, leftName)
		},
		func() (tuple.Color, error) {
			return getColorByName(ctx, rightName)
		},
		func(left, right tuple.Color) tuple.Color {
			return left.MulColor(right)
		},
		fmt.Sprintf("%s * %s", leftName, rightName),
		tuple.NewColor(x, y, z),
	)
}

func negatingVector(
	ctx context.Context,
	name string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, name)
		},
		func() (float64, error) {
			return -1.0, nil
		},
		func(left tuple.Vector, scalar float64) tuple.Vector {
			return left.Scale(scalar)
		},
		fmt.Sprintf("-%s", name),
		tuple.NewVector(x, y, z),
	)
}

func multiplyingVectorByScalar(
	ctx context.Context,
	name string,
	scalar,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, name)
		},
		func() (float64, error) {
			return scalar, nil
		},
		func(left tuple.Vector, scalar float64) tuple.Vector {
			return left.Scale(scalar)
		},
		fmt.Sprintf("%s * %v", name, scalar),
		tuple.NewVector(x, y, z),
	)
}

func multiplyingColorByScalar(
	ctx context.Context,
	name string,
	scalar,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningThreeTuple(
		func() (tuple.Color, error) {
			return getColorByName(ctx, name)
		},
		func() (float64, error) {
			return scalar, nil
		},
		func(left tuple.Color, scalar float64) tuple.Color {
			return left.Scale(scalar)
		},
		fmt.Sprintf("%s * %v", name, scalar),
		tuple.NewColor(x, y, z),
	)
}

func dividingVectorByScalar(
	ctx context.Context,
	name string,
	scalar,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, name)
		},
		func() (float64, error) {
			return scalar, nil
		},
		func(left tuple.Vector, scalar float64) tuple.Vector {
			return left.Div(scalar)
		},
		fmt.Sprintf("%s / %v", name, scalar),
		tuple.NewVector(x, y, z),
	)
}

func vectorMagnitude(
	ctx context.Context,
	name string,
	expected float64,
) error {
	vec, err := getVectorByName(ctx, name)
	if err != nil {
		return err
	}

	got := vec.Magnitude()
	if !math.Equals(math.Epsilon)(expected, got) {
		return fmt.Errorf("for vector %s: expected magnitude %v but got %v",
			name,
			expected,
			got)
	}

	return nil
}

func normalizedVector(
	ctx context.Context,
	name string,
	x,
	y,
	z float64,
) error {
	copy, err := newNormalizedVector(ctx, name)
	if err != nil {
		return err
	}

	return fourTupleEquality(
		fmt.Sprintf("normalize(%s)", name),
		tuple.NewVector(x, y, z),
		copy,
	)
}

func vectorDotProduct(
	ctx context.Context,
	leftName,
	rightName string,
	expected float64,
) error {
	return testBinaryOperationReturningScalar(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, leftName)
		},
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, rightName)
		},
		func(left, right tuple.Vector) float64 {
			return left.Dot(right)
		},
		fmt.Sprintf("dot(%s, %s)", leftName, rightName),
		expected,
	)
}

func vectorCrossProduct(
	ctx context.Context,
	leftName,
	rightName string,
	x,
	y,
	z float64,
) error {
	return testBinaryOperationReturningFourTuple(
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, leftName)
		},
		func() (tuple.Vector, error) {
			return getVectorByName(ctx, rightName)
		},
		func(left, right tuple.Vector) tuple.Vector {
			return left.Cross(right)
		},
		fmt.Sprintf("cross(%s, %s)", leftName, rightName),
		tuple.NewVector(x, y, z),
	)
}

func TestFeatures(t *testing.T) {
	suite := godog.TestSuite{
		ScenarioInitializer: func(sc *godog.ScenarioContext) {
			for _, scenario := range scenarios {
				scenario(sc)
			}
		},
		Options: &godog.Options{
			FS:        features,
			Format:    "pretty",
			Randomize: -1,
			Strict:    true,
			TestingT:  t,
		},
	}

	if code := suite.Run(); code != 0 {
		t.Fatalf("feature test failed with code %d", code)
	}
}
