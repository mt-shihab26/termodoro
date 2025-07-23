package ui

import (
	"fmt"
	"strings"

	"github.com/mt-shihab26/termodoro/internal/config"
)

func getInstructions(timerState TimerState, sessionType SessionType, sessionCount int) string {
	operations := map[bool]string{
		true:  "Pause",
		false: "Start",
	}

	instructionTexts := []string{
		fmt.Sprintf("'space': %s timer", operations[timerState == RunningTimerState]),
		"'r'    : Reset current session",
		fmt.Sprintf("'n'    : Start %s manually", getNextSessionLabel(sessionType, sessionCount)),
		"'q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}

func getNextSessionLabel(sessionType SessionType, sessionCount int) string {
	c := config.Load()

	switch sessionType {
	case WorkSessionType:
		if (sessionCount+1)%c.LongBreakSessionInterval == 0 {
			return "Long Break"
		}
		return "Break"
	case BreakSessionType, LongBreakSessionType:
		return "Work"
	default:
		return ""
	}
}
