// Package session
package session

import "github.com/mt-shihab26/termodoro/view"

type Session struct {
	State view.SessionType
	Count int
}

func New() *Session {
	return &Session{
		State: view.WorkSessionType,
		Count: 0,
	}
}

func (session *Session) NextSession() {
	switch session.State {
	case view.WorkSessionType:
		session.Count++
		if session.Count%4 == 0 {
			session.State = view.LongBreakSessionType
		} else {
			session.State = view.BreakSessionType
		}
	case view.BreakSessionType:
		session.State = view.WorkSessionType
	case view.LongBreakSessionType:
		session.State = view.WorkSessionType
	}
}

func (session *Session) GetDuration() int {
	switch session.State {
	case view.WorkSessionType:
		return 4
	case view.BreakSessionType:
		return 3
	case view.LongBreakSessionType:
		return 2
	default:
		return 0
	}
}
