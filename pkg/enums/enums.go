// Package enums
package enums

type SessionType uint

const (
	WorkSessionType SessionType = iota
	BreakSessionType
	LongBreakSessionType
)

type TimerState uint

const (
	StoppedTimerState TimerState = iota
	RunningTimerState
	PausedTimerState
)
