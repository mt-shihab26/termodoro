// Package root
package root

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/internal/app"
	"github.com/mt-shihab26/termodoro/pkg/commands"
)

func Run() commands.Func {
	return run
}

var run commands.Func = func(args []string) error {
	if err := runTUI(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
	return nil
}

func runTUI() error {
	app := app.New()
	program := tea.NewProgram(
		app,
		tea.WithAltScreen(),
		tea.WithMouseCellMotion(),
	)

	_, err := program.Run()
	return err
}
