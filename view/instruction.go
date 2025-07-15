package view

import (
	"fmt"
	"strings"
)

func getInstructions(timerState TimerState) string {
	operations := map[bool]string{
		true:  "Pause",
		false: "Start",
	}

	instructionTexts := []string{
		fmt.Sprintf("'SPACE': %s timer", operations[timerState == RunningTimerState]),
		"'R'    : Reset current session",
		"'B'    : Start break manually",
		"'Q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}
