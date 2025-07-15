package models

type TimerState uint

const (
	Stopped TimerState = iota
	Running
	Paused
)

type Timer struct {
	State    TimerState
	Duration int // in seconds
	Current  int // current countdown value
}

func NewTimer(duration int) *Timer {
	return &Timer{
		State:    Stopped,
		Duration: duration,
		Current:  duration,
	}
}

func (t *Timer) Start() {
	switch t.State {
	case Stopped:
		t.State = Running
	case Paused:
		t.State = Running
	}
}

func (t *Timer) Pause() {
	if t.State == Running {
		t.State = Paused
	}
}

func (t *Timer) Toggle() {
	switch t.State {
	case Stopped:
		t.Start()
	case Running:
		t.Pause()
	case Paused:
		t.Start()
	}
}

func (t *Timer) Reset() {
	t.State = Stopped
	t.Current = t.Duration
}

func (t *Timer) Tick() {
	if t.State == Running && t.Current > 0 {
		t.Current--
	}
}

func (t *Timer) IsRunning() bool {
	return t.State == Running
}

func (t *Timer) IsFinished() bool {
	return t.Current <= 0
}
