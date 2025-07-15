package app

import tea "github.com/charmbracelet/bubbletea"

func (app *App) handleKeyPress(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "q", "ctrl+c":
		return app, tea.Sequence(tea.ExitAltScreen, tea.Quit)
	case " ":
		app.timer.Toggle()
	case "r":
		app.timer.Reset()
	case "n":
		app.nextSession()
	}
	return app, nil
}
