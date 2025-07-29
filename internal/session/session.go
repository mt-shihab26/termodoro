// Package session
package session

import (
	"fmt"
	"os"
	"time"

	"github.com/mt-shihab26/termodoro/internal/cache"
	"github.com/mt-shihab26/termodoro/internal/config"
	"github.com/mt-shihab26/termodoro/internal/ui"
	"github.com/mt-shihab26/termodoro/internal/utils"
)

type Session struct {
	State    ui.SessionType
	Count    int
	LastDate string
}

func New() *Session {
	data, err := cache.Load()
	if err != nil {
		session := &Session{
			State:    ui.WorkSessionType,
			Count:    0,
			LastDate: getCurrentDate(),
		}
		err := cache.Save(&cache.PCache{
			SessionType:     &session.State,
			SessionCount:    &session.Count,
			SessionLastDate: &session.LastDate,
		})
		if err != nil {
			fmt.Fprintf(os.Stderr, "Warning: failed to save initial session to cache: %v\n", err)
		}
		return session
	}
	seesion := &Session{
		State:    data.SessionType,
		Count:    data.SessionCount,
		LastDate: data.SessionLastDate,
	}

	seesion.ResetIfNewDay()

	return seesion
}

func (session *Session) NextSession() {
	session.ResetIfNewDay()

	cfg := config.Load()

	message := ""

	switch session.State {
	case ui.WorkSessionType:
		session.Count++
		if session.Count%cfg.LongBreakSessionInterval == 0 {
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

	session.LastDate = getCurrentDate()

	err := cache.Save(&cache.PCache{
		SessionType:     &session.State,
		SessionCount:    &session.Count,
		SessionLastDate: &session.LastDate,
	})
	if err != nil {
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

func (session *Session) ResetIfNewDay() {
	if getCurrentDate() == session.LastDate {
		return
	}

	session.Count = 0
	session.LastDate = getCurrentDate()

	if err := cache.Save(&cache.PCache{
		SessionType:     &session.State,
		SessionCount:    &session.Count,
		SessionLastDate: &session.LastDate,
	}); err != nil {
		fmt.Fprintf(os.Stderr, "Warning: failed to save session reset to cache: %v\n", err)
	}
}

func getCurrentDate() string {
	return time.Now().Format("2006-01-02")
}
