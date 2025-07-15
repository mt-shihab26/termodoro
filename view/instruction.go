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
		"'SPACE': Start/Pause timer",
		"'R'    : Reset current session",
		"'B'    : Start break manually",
		"'N'    : Skip to next session",
		"'Q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}
