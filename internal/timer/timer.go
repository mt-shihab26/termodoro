// Package timer
package timer

import "github.com/mt-shihab26/termodoro/internal/ui"

type Timer struct {
	State    ui.TimerState
	Duration int
	Current  int
}

func New(duration int) *Timer {
	return &Timer{
		State:    ui.StoppedTimerState,
		Duration: duration,
		Current:  duration,
	}
}

func (t *Timer) Tick() {
	if t.State == ui.RunningTimerState && t.Current > 0 {
		t.Current--
	}
}

func (t *Timer) Toggle() {
	switch t.State {
	case ui.StoppedTimerState:
		t.Start()
	case ui.RunningTimerState:
		t.pause()
	case ui.PausedTimerState:
		t.Start()
	}
}

func (t *Timer) Start() {
	switch t.State {
	case ui.StoppedTimerState:
		t.State = ui.RunningTimerState
	case ui.PausedTimerState:
		t.State = ui.RunningTimerState
	}
}

func (t *Timer) Reset() {
	t.State = ui.StoppedTimerState
	t.Current = t.Duration
}

func (t *Timer) IsFinished() bool {
	return t.Current <= 0
}

func (t *Timer) pause() {
	if t.State == ui.RunningTimerState {
		t.State = ui.PausedTimerState
	}
}
