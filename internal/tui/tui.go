// Package tui
package tui

import (
	tea "github.com/charmbracelet/bubbletea"
)

func Run() error {
	app := NewApp()

	program := tea.NewProgram(
		app,
		tea.WithAltScreen(),
		tea.WithMouseCellMotion(),
	)

	_, err := program.Run()

	return err
}
