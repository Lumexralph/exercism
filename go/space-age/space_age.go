package space

// Planet a custom type for naming any planet
type Planet string

/* The time of earth in secs or days in relation
to other planets.

Earth: orbital period 365.25 Earth days, or 31557600 seconds or 1 year.
Mercury: orbital period 0.2408467 Earth years.
Venus: orbital period 0.61519726 Earth years.
Mars: orbital period 1.8808158 Earth years.
Jupiter: orbital period 11.862615 Earth years.
Saturn: orbital period 29.447498 Earth years.
Uranus: orbital period 84.016846 Earth years.
Neptune: orbital period 164.79132 Earth years.
*/
const timeOnEarth = 31557600.00 // in a year on earth

// Age in seconds, calculate how old someone would be on different planets
func Age(secs float64, name Planet) float64 {
	planets := map[Planet]float64 {
		"Earth": 1.0,
		"Mercury": 0.2408467,
		"Venus": 0.61519726,
		"Mars": 1.8808158,
		"Jupiter": 11.862615,
		"Saturn": 29.447498,
		"Uranus": 84.016846,
		"Neptune": 164.79132,
	}

	// calculate the number of earth years in secs
	years := secs / (planets[name] * timeOnEarth)

	return years
}

