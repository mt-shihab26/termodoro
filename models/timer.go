package models

// TimerState represents the current state of the timer
type TimerState uint

const (
	Stopped TimerState = iota
	Running
	Paused
)

// Timer represents the timer model
type Timer struct {
	State    TimerState
	Duration int // in seconds
	Current  int // current countdown value
}

// NewTimer creates a new timer with the specified duration
func NewTimer(duration int) *Timer {
	return &Timer{
		State:    Stopped,
		Duration: duration,
		Current:  duration,
	}
}

// Start starts the timer
func (t *Timer) Start() {
	if t.State == Stopped {
		t.State = Running
	} else if t.State == Paused {
		t.State = Running
	}
}

// Pause pauses the timer
func (t *Timer) Pause() {
	if t.State == Running {
		t.State = Paused
	}
}

// Toggle toggles between running and paused states
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

// Reset resets the timer to its initial state
func (t *Timer) Reset() {
	t.State = Stopped
	t.Current = t.Duration
}

// Tick decrements the timer by one second if running
func (t *Timer) Tick() {
	if t.State == Running && t.Current > 0 {
		t.Current--
	}
}

// IsRunning returns true if the timer is currently running
func (t *Timer) IsRunning() bool {
	return t.State == Running
}

// IsFinished returns true if the timer has reached zero
func (t *Timer) IsFinished() bool {
	return t.Current <= 0
}
