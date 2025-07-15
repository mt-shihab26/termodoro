// Package app provides main application functionality for the Pomodoro application.
package app

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/config"
	"github.com/mt-shihab26/termodoro/internal/timer"
	"github.com/mt-shihab26/termodoro/view"
)

type App struct {
	timer  *timer.Timer
	width  int
	height int
}

func New(_ *config.Config) *App {
	return &App{
		timer: timer.New(60),
	}
}

func (app *App) Init() tea.Cmd {
	return tea.Batch(
		tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return time.Time(t)
		}),
		tea.EnterAltScreen,
	)
}

func (app *App) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		app.width = msg.Width
		app.height = msg.Height
		return app, nil
	case time.Time:
		app.timer.Tick()
		if app.timer.IsFinished() {
		}
		return app, tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return time.Time(t)
		})
	case tea.KeyMsg:
		return app.handleKeyPress(msg)
	}
	return app, nil
}

func (app *App) View() string {
	return view.Render(view.Data{
		Width:       app.width,
		Height:      app.height,
		SessionType: view.WorkSessionType,
		TimerState:  app.timer.State,
		CurrentTime: app.timer.Current,
	})
}
