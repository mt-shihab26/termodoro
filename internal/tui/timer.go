package tui

import "github.com/mt-shihab26/termodoro/pkg/enums"

type Timer struct {
	State    enums.TimerState
	Duration int
	Current  int
}

func NewTimer(duration int) *Timer {
	return &Timer{
		State:    enums.StoppedTimerState,
		Duration: duration,
		Current:  duration,
	}
}

func (t *Timer) Tick() {
	if t.State == enums.RunningTimerState && t.Current > 0 {
		t.Current--
	}
}

func (t *Timer) Toggle() {
	switch t.State {
	case enums.StoppedTimerState:
		t.Start()
	case enums.RunningTimerState:
		t.pause()
	case enums.PausedTimerState:
		t.Start()
	}
}

func (t *Timer) Start() {
	switch t.State {
	case enums.StoppedTimerState:
		t.State = enums.RunningTimerState
	case enums.PausedTimerState:
		t.State = enums.RunningTimerState
	}
}

func (t *Timer) Reset() {
	t.State = enums.StoppedTimerState
	t.Current = t.Duration
}

func (t *Timer) IsFinished() bool {
	return t.Current <= 0
}

func (t *Timer) pause() {
	if t.State == enums.RunningTimerState {
		t.State = enums.PausedTimerState
	}
}
