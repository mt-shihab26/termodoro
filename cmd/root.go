// Package cmd
package cmd

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/cmd/help"
	"github.com/mt-shihab26/termodoro/cmd/version"
	"github.com/mt-shihab26/termodoro/internal/app"
)

func Execute() {
	if err := executeCommand(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func executeCommand() error {
	args := os.Args[1:]

	// Handle flags first
	if len(args) > 0 {
		switch args[0] {
		case "-v", "--version":
			return version.Run(args[1:])
		case "-h", "--help":
			return help.Run(args[1:])
		}
	}

	// Handle subcommands
	if len(args) > 0 {
		switch args[0] {
		case "version":
			return version.Run(args[1:])
		case "help":
			return help.Run(args[1:])
		default:
			fmt.Fprintf(os.Stderr, "Unknown command: %s\n", args[0])
			fmt.Fprintf(os.Stderr, "Run 'termodoro help' for usage information.\n")
			os.Exit(1)
		}
	}

	// Run TUI (default)
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

func SetVersion(ver, commit, date string) {
	version.Version = ver
	version.Commit = commit
	version.Date = date
}
