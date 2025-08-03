package tui

import (
	"fmt"
	"strings"

	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
	"github.com/mt-shihab26/termodoro/pkg/config"
	"github.com/mt-shihab26/termodoro/pkg/enums"
)

type View struct {
	Width        int
	Height       int
	SessionType  enums.SessionType
	SessionCount int
	TimerState   enums.TimerState
	CurrentTime  int
}

func Render(view View) string {
	if view.Width == 0 || view.Height == 0 {
		return "Loading..."
	}
	return lipgloss.NewStyle().
		Width(view.Width).
		Height(view.Height).
		Align(lipgloss.Center, lipgloss.Center).
		Render(lipgloss.JoinVertical(
			lipgloss.Center,
			lipgloss.NewStyle().
				Foreground(getTimerColor(view.TimerState, view.SessionType)).
				Bold(true).
				Render(getSessionInfo(view.SessionType, view.SessionCount)),
			lipgloss.NewStyle().
				Bold(true).
				Foreground(getTimerColor(view.TimerState, view.SessionType)).
				Padding(2, 2, 2, 2).
				Align(lipgloss.Center).
				Render(getTimerInfo(view.CurrentTime)),
			lipgloss.NewStyle().
				Foreground(lipgloss.Color("240")).
				Render(getInstructions(view.TimerState, view.SessionType, view.SessionCount)),
		))
}

func getInstructions(timerState enums.TimerState, sessionType enums.SessionType, sessionCount int) string {
	operations := map[bool]string{
		true:  "Pause",
		false: "Start",
	}

	instructionTexts := []string{
		fmt.Sprintf("'space': %s timer", operations[timerState == enums.RunningTimerState]),
		"'r'    : Reset current session",
		fmt.Sprintf("'n'    : Start %s manually", getNextSessionLabel(sessionType, sessionCount)),
		"'q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}

func getNextSessionLabel(sessionType enums.SessionType, sessionCount int) string {
	c := config.Load()

	switch sessionType {
	case enums.WorkSessionType:
		if (sessionCount+1)%c.LongBreakSessionInterval == 0 {
			return "Long Break"
		}
		return "Break"
	case enums.BreakSessionType, enums.LongBreakSessionType:
		return "Work"
	default:
		return ""
	}
}

func getSessionInfo(sessionType enums.SessionType, sessionCount int) string {
	var sessionName string

	switch sessionType {
	case enums.WorkSessionType:
		sessionName = "Work"
	case enums.BreakSessionType:
		sessionName = "Break"
	case enums.LongBreakSessionType:
		sessionName = "Long Break"
	}

	return fmt.Sprintf("%s | Completed Today: %d", sessionName, sessionCount)
}

func getSessionColor(sessionType enums.SessionType) lipgloss.Color {
	switch sessionType {
	case enums.WorkSessionType:
		return lipgloss.Color("86") // Green
	case enums.BreakSessionType:
		return lipgloss.Color("39") // Blue
	case enums.LongBreakSessionType:
		return lipgloss.Color("129") // Purple
	default:
		return lipgloss.Color("240") // Gray
	}
}

func getTimerColor(timerState enums.TimerState, sessionType enums.SessionType) lipgloss.Color {
	switch timerState {
	case enums.RunningTimerState:
		return getSessionColor(sessionType)
	case enums.PausedTimerState:
		return lipgloss.Color("208") // Orange
	default:
		return lipgloss.Color("240") // Gray
	}
}

func getTimerInfo(currentTime int) string {
	// Format time as MM:SS
	minutes := currentTime / 60
	seconds := currentTime % 60
	timeStr := fmt.Sprintf("%02d:%02d", minutes, seconds)
	return figure.NewFigure(timeStr, "big", true).String()
}
