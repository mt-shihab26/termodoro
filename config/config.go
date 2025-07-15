package config

type Config struct {
	WorkDuration            int
	ShortBreakDuration      int
	LongBreakDuration       int
	SessionsBeforeLongBreak int
}

func New() *Config {
	return &Config{
		WorkDuration:            55,
		ShortBreakDuration:      5,
		LongBreakDuration:       15,
		SessionsBeforeLongBreak: 4,
	}
}
