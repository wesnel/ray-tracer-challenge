package number

type Interval struct {
	Min float64
	Max float64
}

func (i Interval) Clamp(number float64) float64 {
	if number < i.Min {
		return i.Min
	}

	if number > i.Max {
		return i.Max
	}

	return number
}

func ChangeInterval(number float64, old, new Interval) float64 {
	return new.Min + (((new.Max - new.Min) / (old.Max - old.Min)) * (number - old.Min))
}
