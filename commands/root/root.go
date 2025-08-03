// Package root
package root

import (
	"fmt"
	"os"

	"github.com/mt-shihab26/termodoro/internal/tui"
	"github.com/mt-shihab26/termodoro/pkg/commands"
)

func Run() commands.Func {
	return run
}

var run commands.Func = func(args []string) error {
	if err := tui.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
	return nil
}
