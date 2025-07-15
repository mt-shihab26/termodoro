package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/internal/app"
)

func main() {
	app := app.New()

	program := tea.NewProgram(
		app,
		tea.WithAltScreen(),
		tea.WithMouseCellMotion(),
	)

	_, err := program.Run()
	if err != nil {
		panic(err)
	}
}
