package ui

import (
	"fmt"
	"strings"
)

func getInstructions(timerState TimerState, sessionType SessionType) string {
	operations := map[bool]string{
		true:  "Pause",
		false: "Start",
	}
	sessions := map[SessionType]string{
		WorkSessionType:      "work",
		BreakSessionType:     "break",
		LongBreakSessionType: "long break",
	}

	instructionTexts := []string{
		fmt.Sprintf("'space': %s timer", operations[timerState == RunningTimerState]),
		"'r'    : Reset current session",
		fmt.Sprintf("'n'    : Start %s manually", sessions[sessionType]),
		"'q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}
