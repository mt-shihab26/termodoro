package ui

import (
	"fmt"

	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
)

type TimerState uint

const (
	StoppedTimerState TimerState = iota
	RunningTimerState
	PausedTimerState
)

func getTimerColor(timerState TimerState, sessionType SessionType) lipgloss.Color {
	switch timerState {
	case RunningTimerState:
		return getSessionColor(sessionType)
	case PausedTimerState:
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
