package models

type SessionType uint

const (
	Work SessionType = iota
	ShortBreak
	LongBreak
)

type Session struct {
	Type      SessionType
	Duration  int
	Completed int
}

type SessionManager struct {
	Sessions                []Session
	CurrentSession          int
	WorkDuration            int
	ShortBreakDuration      int
	LongBreakDuration       int
	SessionsBeforeLongBreak int
}

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

func (sm *SessionManager) GetCurrentSessionDuration() int {
	completedWorkSessions := sm.getCompletedWorkSessions()

	// If we have completed work sessions and the last session was work, we're in a break
	if len(sm.Sessions) > 0 && sm.Sessions[len(sm.Sessions)-1].Type == Work {
		if completedWorkSessions > 0 && completedWorkSessions%sm.SessionsBeforeLongBreak == 0 {
			return sm.LongBreakDuration
		}
		return sm.ShortBreakDuration
	}

	// Otherwise, we're in a work session
	return sm.WorkDuration
}

func (sm *SessionManager) getCompletedWorkSessions() int {
	count := 0
	for _, session := range sm.Sessions {
		if session.Type == Work {
			count++
		}
	}
	return count
}

// GetStats returns statistics about completed sessions
func (sm *SessionManager) GetStats() (workSessions, shortBreaks, longBreaks int, totalWorkTime, totalBreakTime int) {
	for _, session := range sm.Sessions {
		switch session.Type {
		case Work:
			workSessions++
			totalWorkTime += session.Completed
		case ShortBreak:
			shortBreaks++
			totalBreakTime += session.Completed
		case LongBreak:
			longBreaks++
			totalBreakTime += session.Completed
		}
	}
	return
}

// GetTotalSessions returns the total number of completed sessions
func (sm *SessionManager) GetTotalSessions() int {
	return len(sm.Sessions)
}

// GetLastSession returns the last completed session, or nil if none exist
func (sm *SessionManager) GetLastSession() *Session {
	if len(sm.Sessions) == 0 {
		return nil
	}
	return &sm.Sessions[len(sm.Sessions)-1]
}

// IsTimeForLongBreak checks if the next break should be a long break
func (sm *SessionManager) IsTimeForLongBreak() bool {
	completedWorkSessions := sm.getCompletedWorkSessions()
	return completedWorkSessions > 0 && completedWorkSessions%sm.SessionsBeforeLongBreak == 0
}

// Reset clears all session history
func (sm *SessionManager) Reset() {
	sm.Sessions = make([]Session, 0)
	sm.CurrentSession = 0
}
