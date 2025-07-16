// Package session
package session

import (
	"fmt"
	"os"

	"github.com/mt-shihab26/termodoro/storage/cache"
	"github.com/mt-shihab26/termodoro/view"
)

type Session struct {
	State view.SessionType
	Count int
}

func New() *Session {
	data, err := cache.Load()
	if err != nil {
		session := &Session{
			State: view.WorkSessionType,
			Count: 0,
		}
		err := cache.Save(cache.New(session.State, session.Count))
		if err != nil {
			fmt.Fprintf(os.Stderr, "Warning: failed to save initial session to cache: %v\n", err)
		}
		return session
	}
	return &Session{
		State: data.SessionType,
		Count: data.SessionCount,
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

	err := cache.Save(cache.New(session.State, session.Count))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Warning: failed to save session to cache: %v\n", err)
	}
}

func (session *Session) GetDuration() int {
	switch session.State {
	case view.WorkSessionType:
		return 6
	case view.LongBreakSessionType:
		return 4
	case view.BreakSessionType:
		return 2
	default:
		return 0
	}
}
