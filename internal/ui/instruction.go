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
		fmt.Sprintf("'SPACE': %s timer", operations[timerState == RunningTimerState]),
		"'R'    : Reset current session",
		fmt.Sprintf("'N'    : Start %s manually", sessions[sessionType]),
		"'Q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}
