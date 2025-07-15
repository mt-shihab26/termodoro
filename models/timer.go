package models

import "github.com/mt-shihab26/termodoro/view"

type Timer struct {
	State    view.TimerState
	Duration int // in seconds
	Current  int // current countdown value
}

func NewTimer(duration int) *Timer {
	return &Timer{
		State:    view.StoppedTimerState,
		Duration: duration,
		Current:  duration,
	}
}

func (t *Timer) Start() {
	switch t.State {
	case view.StoppedTimerState:
		t.State = view.RunningTimerState
	case view.PausedTimerState:
		t.State = view.RunningTimerState
	}
}

func (t *Timer) Pause() {
	if t.State == view.RunningTimerState {
		t.State = view.PausedTimerState
	}
}

func (t *Timer) Toggle() {
	switch t.State {
	case view.StoppedTimerState:
		t.Start()
	case view.RunningTimerState:
		t.Pause()
	case view.PausedTimerState:
		t.Start()
	}
}

func (t *Timer) Reset() {
	t.State = view.StoppedTimerState
	t.Current = t.Duration
}

func (t *Timer) Tick() {
	if t.State == view.RunningTimerState && t.Current > 0 {
		t.Current--
	}
}

func (t *Timer) IsRunning() bool {
	return t.State == view.RunningTimerState
}

func (t *Timer) IsFinished() bool {
	return t.Current <= 0
}
