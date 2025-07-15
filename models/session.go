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

	if completedWorkSessions > 0 && completedWorkSessions%sm.SessionsBeforeLongBreak == 0 {
		return sm.LongBreakDuration
	}

	if len(sm.Sessions) > 0 && sm.Sessions[len(sm.Sessions)-1].Type == Work {
		return sm.ShortBreakDuration
	}

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
