package models

// SessionType represents the type of session
type SessionType uint

const (
	Work SessionType = iota
	ShortBreak
	LongBreak
)

// Session represents a pomodoro session
type Session struct {
	Type      SessionType
	Duration  int
	Completed int
}

// SessionManager manages pomodoro sessions
type SessionManager struct {
	Sessions         []Session
	CurrentSession   int
	WorkDuration     int
	ShortBreakDuration int
	LongBreakDuration  int
	SessionsBeforeLongBreak int
}

// NewSessionManager creates a new session manager
func NewSessionManager(workDuration, shortBreakDuration, longBreakDuration, sessionsBeforeLongBreak int) *SessionManager {
	return &SessionManager{
		Sessions:                make([]Session, 0),
		CurrentSession:          0,
		WorkDuration:            workDuration,
		ShortBreakDuration:      shortBreakDuration,
		LongBreakDuration:       longBreakDuration,
		SessionsBeforeLongBreak: sessionsBeforeLongBreak,
	}
}

// GetCurrentSessionDuration returns the duration for the current session type
func (sm *SessionManager) GetCurrentSessionDuration() int {
	completedWorkSessions := sm.getCompletedWorkSessions()
	
	if completedWorkSessions > 0 && completedWorkSessions%sm.SessionsBeforeLongBreak == 0 {
		return sm.LongBreakDuration
	}
	
	if len(sm.Sessions) > 0 && sm.Sessions[len(sm.Sessions)-1].Type == Work {
		return sm.ShortBreakDuration
	}
	
	return sm.WorkDuration
}

// getCompletedWorkSessions returns the number of completed work sessions
func (sm *SessionManager) getCompletedWorkSessions() int {
	count := 0
	for _, session := range sm.Sessions {
		if session.Type == Work {
			count++
		}
	}
	return count
}
