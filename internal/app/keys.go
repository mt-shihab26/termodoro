package app

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/storage/cache"
)

func (app *App) handleKeyPress(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "q", "ctrl+c":
		cache.Save(&cache.PCache{TimerCurrent: &app.timer.Current})
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
