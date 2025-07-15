package models

import "github.com/mt-shihab26/termodoro/view"

type Session struct {
	Type      view.SessionType
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

	if len(sm.Sessions) > 0 && sm.Sessions[len(sm.Sessions)-1].Type == view.WorkSessionType {
		if completedWorkSessions > 0 && completedWorkSessions%sm.SessionsBeforeLongBreak == 0 {
			return sm.LongBreakDuration
		}
		return sm.ShortBreakDuration
	}

	return sm.WorkDuration
}

func (sm *SessionManager) getCompletedWorkSessions() int {
	count := 0
	for _, session := range sm.Sessions {
		if session.Type == view.WorkSessionType {
			count++
		}
	}
	return count
}

func (sm *SessionManager) GetStats() (workSessions, shortBreaks, longBreaks int, totalWorkTime, totalBreakTime int) {
	for _, session := range sm.Sessions {
		switch session.Type {
		case view.WorkSessionType:
			workSessions++
			totalWorkTime += session.Completed
		case view.BreakSessionType:
			shortBreaks++
			totalBreakTime += session.Completed
		case view.LongBreakSessionType:
			longBreaks++
			totalBreakTime += session.Completed
		}
	}
	return
}

func (sm *SessionManager) GetTotalSessions() int {
	return len(sm.Sessions)
}

func (sm *SessionManager) GetLastSession() *Session {
	if len(sm.Sessions) == 0 {
		return nil
	}
	return &sm.Sessions[len(sm.Sessions)-1]
}

func (sm *SessionManager) IsTimeForLongBreak() bool {
	completedWorkSessions := sm.getCompletedWorkSessions()
	return completedWorkSessions > 0 && completedWorkSessions%sm.SessionsBeforeLongBreak == 0
}

func (sm *SessionManager) Reset() {
	sm.Sessions = make([]Session, 0)
	sm.CurrentSession = 0
}
