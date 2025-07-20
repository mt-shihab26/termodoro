// Package session
package session

import (
	"fmt"
	"os"

	"github.com/mt-shihab26/termodoro/internal/cache"
	"github.com/mt-shihab26/termodoro/internal/config"
	"github.com/mt-shihab26/termodoro/internal/ui"
	"github.com/mt-shihab26/termodoro/internal/utils"
)

type Session struct {
	State ui.SessionType
	Count int
}

func New() *Session {
	data, err := cache.Load()
	if err != nil {
		session := &Session{
			State: ui.WorkSessionType,
			Count: 0,
		}
		err := cache.Save(&cache.PCache{
			SessionType:  &session.State,
			SessionCount: &session.Count,
		})
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
	message := ""

	switch session.State {
	case ui.WorkSessionType:
		session.Count++
		if session.Count%4 == 0 {
			session.State = ui.LongBreakSessionType
			message = "Time for a long break!"
		} else {
			session.State = ui.BreakSessionType
			message = "Time for a break!"
		}
	case ui.BreakSessionType, ui.LongBreakSessionType:
		session.State = ui.WorkSessionType
		message = "Time to get back to work!"
	}

	if err := cache.Save(&cache.PCache{SessionType: &session.State, SessionCount: &session.Count}); err != nil {
		fmt.Fprintf(os.Stderr, "Warning: failed to save session to cache: %v\n", err)
	}

	if message != "" {
		if err := utils.NotifyWithSound("Termodoro", message, ""); err != nil {
			fmt.Fprintf(os.Stderr, "Warning: failed to send notification: %v\n", err)
		}
	}
}

func (session *Session) GetDuration() int { // in seconds
	timerCurrent, err := cache.LoadTimerCurrent()
	if err != nil {
		return session.getDefaultDuration()
	}
	return timerCurrent
}

func (session *Session) getDefaultDuration() int { // in seconds
	cfg := config.Load()
	switch session.State {
	case ui.WorkSessionType:
		return cfg.WorkSessionDuration * 60
	case ui.BreakSessionType:
		return cfg.BreakSessionDuration * 60
	case ui.LongBreakSessionType:
		return cfg.LongBreakSessionDuration * 60
	default:
		return 0
	}
}
