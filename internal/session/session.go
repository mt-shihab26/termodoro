package session

import "github.com/mt-shihab26/termodoro/view"

type Session struct {
	State view.SessionType
}

func New() *Session {
	return &Session{
		State: view.WorkSessionType,
	}
}

func (session *Session) NextSession() {

}

func (session *Session) GetDuration() int {
	return 5
}
