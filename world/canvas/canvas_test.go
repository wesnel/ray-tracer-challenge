package canvas_test

import (
	"context"
	"embed"
	"fmt"
	"testing"

	"github.com/cucumber/godog"

	"git.sr.ht/~wgn/ray-tracer-challenge/math"
	"git.sr.ht/~wgn/ray-tracer-challenge/math/tuple"
	"git.sr.ht/~wgn/ray-tracer-challenge/world/canvas"
)

// embed the cucumber feature files.  this is probably a little bit
// more safe/portable/fast than just providing the folder name to
// godog:
//
//go:embed features/*.feature
var features embed.FS

// functions to get all the channels from a color:
var channels = map[string]func(tuple.Color) float64{
	"red":   func(c tuple.Color) float64 { return c.Red() },
	"green": func(c tuple.Color) float64 { return c.Green() },
	"blue":  func(c tuple.Color) float64 { return c.Blue() },
}

// functions to get properties of a canvas:
var canvasGetters = map[string]func(canvas.Canvas) uint64{
	"width":  func(c canvas.Canvas) uint64 { return c.Width() },
	"height": func(c canvas.Canvas) uint64 { return c.Height() },
}

// initialization functions for the test scenarios:
var scenarios = []func(*godog.ScenarioContext){
	colors,
	canvases,
}

type ctxKey string

func colors(sc *godog.ScenarioContext) {
	// color creation:
	sc.Given(
		fmt.Sprintf(`^(\w+) <- color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		givenColor)
}

func canvases(sc *godog.ScenarioContext) {
	// canvas creation:
	sc.Given(
		`^(\w+) <- canvas\((\d+), (\d+)\)$`,
		givenCanvas)

	// canvas field validation:
	for field, getter := range canvasGetters {
		sc.Step(
			fmt.Sprintf(`^(\w+)\.%s = (\d+)$`,
				field),
			canvasHasValue(field, getter))
	}

	// writing pixels:
	sc.When(
		`^write_pixel\((\w+), (\d+), (\d+), (\w+)\)$`,
		writePixel)

	// pixel validation:
	sc.Step(
		fmt.Sprintf(`^every pixel of (\w+) is color\(%s, %s, %s\)$`,
			math.FloatFormat,
			math.FloatFormat,
			math.FloatFormat),
		everyPixel)
	sc.Step(
		`^pixel_at\((\w+), (\d+), (\d+)\) = (\w+)$`,
		pixelAt)
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

func givenCanvas(
	ctx context.Context,
	name string,
	width,
	height int,
) (context.Context, error) {
	return context.WithValue(ctx, ctxKey(name), canvas.New(uint64(width), uint64(height))), nil
}

func writePixel(
	ctx context.Context,
	canvasName string,
	x,
	y int,
	colorName string,
) (context.Context, error) {
	got, err := getCanvasByName(ctx, canvasName)
	if err != nil {
		return ctx, err
	}

	color, err := getColorByName(ctx, colorName)
	if err != nil {
		return ctx, err
	}

	got.Set(uint64(x), uint64(y), color)
	return ctx, err
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

func getCanvasByName(
	ctx context.Context,
	name string,
) (canvas.Canvas, error) {
	got, ok := ctx.Value(ctxKey(name)).(canvas.Canvas)
	if !ok {
		return got, fmt.Errorf("invalid canvas variable name %s",
			name)
	}

	return got, nil
}

func canvasHasValue(
	field string,
	getter func(canvas.Canvas) uint64,
) func(
	ctx context.Context,
	name string,
	expected int,
) error {
	return func(
		ctx context.Context,
		name string,
		expected int,
	) error {
		got, err := getCanvasByName(ctx, name)
		if err != nil {
			return err
		}

		return compareValues(
			name,
			field,
			uint64(expected),
			getter(got),
		)
	}
}

func everyPixel(
	ctx context.Context,
	canvasName string,
	red,
	green,
	blue float64,
) error {
	got, err := getCanvasByName(ctx, canvasName)
	if err != nil {
		return err
	}

	color := tuple.NewColor(red, green, blue)

	for i, item := range got.Contents() {
		if err := colorEquality(
			fmt.Sprintf("at index %d", i),
			color,
			item.(tuple.Color),
		); err != nil {
			return err
		}
	}

	return nil
}

func pixelAt(
	ctx context.Context,
	canvasName string,
	x,
	y int,
	colorName string,
) error {
	got, err := getCanvasByName(ctx, canvasName)
	if err != nil {
		return err
	}

	color, err := getColorByName(ctx, colorName)
	if err != nil {
		return err
	}

	value := got.Get(uint64(x), uint64(y)).(tuple.Color)

	return colorEquality(
		fmt.Sprintf("at (%d, %d)", x, y),
		color,
		value,
	)
}

func compareValues(
	name,
	field string,
	expected,
	got uint64,
) error {
	if expected != got {
		return fmt.Errorf("for %s of canvas %s: expected %d but got %d",
			field,
			name,
			expected,
			got)
	}

	return nil
}

func colorEquality(
	name string,
	expected,
	got tuple.Color,
) error {
	for field, getter := range channels {
		if err := compareColorValues(
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

func compareColorValues(
	name,
	field string,
	expected,
	got float64,
) error {
	if !math.Equals(math.Epsilon)(expected, got) {
		return fmt.Errorf("for %s of color %s: expected %v but got %v",
			field,
			name,
			expected,
			got)
	}

	return nil
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
