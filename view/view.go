package view

import (
	"github.com/charmbracelet/lipgloss"
)

type Data struct {
	Width       int
	Height      int
	SessionType SessionType
	TimerState  TimerState
	CurrentTime int
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
				Foreground(lipgloss.Color("33")).
				Bold(true).
				MarginBottom(1).
				Render(getSessionInfo(data.SessionType)),
			lipgloss.NewStyle().
				Bold(true).
				Foreground(getTimerColor(data.TimerState, data.SessionType)).
				Padding(1, 2).
				Align(lipgloss.Center).
				Render(getTimerInfo(data.CurrentTime)),
			lipgloss.NewStyle().
				Foreground(lipgloss.Color("240")).
				MarginTop(2).
				Render(getInstructions(data.TimerState)),
		))
}
