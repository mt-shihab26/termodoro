package tui

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/pkg/cache"
)

type App struct {
	timer   *Timer
	session *Session
	width   int
	height  int
}

func NewApp() *App {
	s := NewSession()
	t := NewTimer(s.GetDuration())

	return &App{
		timer:   t,
		session: s,
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
			app.nextSession()
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
	return Render(View{
		Width:        app.width,
		Height:       app.height,
		SessionType:  app.session.State,
		SessionCount: app.session.Count,
		TimerState:   app.timer.State,
		CurrentTime:  app.timer.Current,
	})
}

func (app *App) handleKeyPress(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "q", "ctrl+c":
		cache.Save(&cache.PCache{TimerCurrent: &app.timer.Current})
		return app, tea.Sequence(tea.ExitAltScreen, tea.Quit)
	case " ":
		app.timer.Toggle()
	case "r":
		// app.timer.Reset()
	case "n":
		app.nextSession()
	}
	return app, nil
}

func (app *App) nextSession() {
	app.session.NextSession()
	app.timer = NewTimer(app.session.GetDuration())
}
