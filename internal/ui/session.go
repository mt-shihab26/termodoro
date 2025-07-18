package ui

import (
	"fmt"

	"github.com/charmbracelet/lipgloss"
)

type SessionType uint

const (
	WorkSessionType SessionType = iota
	BreakSessionType
	LongBreakSessionType
)

func getSessionInfo(sessionType SessionType, sessionCount int) string {
	var sessionName string

	switch sessionType {
	case WorkSessionType:
		sessionName = "Work"
	case BreakSessionType:
		sessionName = "Break"
	case LongBreakSessionType:
		sessionName = "Long Break"
	}

	return fmt.Sprintf("%s | Completed Today: %d", sessionName, sessionCount)
}

func getSessionColor(sessionType SessionType) lipgloss.Color {
	switch sessionType {
	case WorkSessionType:
		return lipgloss.Color("86") // Green
	case BreakSessionType:
		return lipgloss.Color("39") // Blue
	case LongBreakSessionType:
		return lipgloss.Color("129") // Purple
	default:
		return lipgloss.Color("240") // Gray
	}
}
