package root

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/internal/app"
	"github.com/mt-shihab26/termodoro/internal/commands/help"
	"github.com/mt-shihab26/termodoro/internal/commands/version"
)

func Execute() {
	if err := executeCommand(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func SetVersion(ver, commit, date string) {
	version.Version = ver
	version.Commit = commit
	version.Date = date
}

func executeCommand() error {
	args := os.Args[1:]

	if len(args) > 0 {
		switch args[0] {
		case "version", "-v", "--version":
			return version.Run(args[1:])
		case "help", "-h", "--help":
			return help.Run(args[1:])
		}
	}

	return runTUI()
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
