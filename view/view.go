// Package view
package view

import (
	"github.com/charmbracelet/lipgloss"
)

type Data struct {
	Width        int
	Height       int
	SessionType  SessionType
	SessionCount int
	TimerState   TimerState
	CurrentTime  int
}

func Render(data Data) string {
	if data.Width == 0 || data.Height == 0 {
		return "Loading..."
	}
	return lipgloss.NewStyle().
		Width(data.Width).
		Height(data.Height).
		Align(lipgloss.Center, lipgloss.Center).
		Render(lipgloss.JoinVertical(
			lipgloss.Center,
			lipgloss.NewStyle().
				Foreground(getTimerColor(data.TimerState, data.SessionType)).
				Bold(true).
				Render(getSessionInfo(data.SessionType, data.SessionCount)),
			lipgloss.NewStyle().
				Bold(true).
				Foreground(getTimerColor(data.TimerState, data.SessionType)).
				Padding(2, 2, 2, 2).
				Align(lipgloss.Center).
				Render(getTimerInfo(data.CurrentTime)),
			lipgloss.NewStyle().
				Foreground(lipgloss.Color("240")).
				Render(getInstructions(data.TimerState, data.SessionType)),
		))
}
