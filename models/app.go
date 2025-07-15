package models

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/config"
	"github.com/mt-shihab26/termodoro/view"
)

type TickMsg time.Time

type AppModel struct {
	timer          *Timer
	sessionManager *SessionManager
	width          int
	height         int
}

func NewAppModel(c *config.Config) *AppModel {
	sm := NewSessionManager(c.WorkDuration, c.ShortBreakDuration, c.LongBreakDuration, c.SessionsBeforeLongBreak)
	initialDuration := sm.GetCurrentSessionDuration()

	return &AppModel{
		timer:          NewTimer(initialDuration),
		sessionManager: sm,
	}
}

func (m *AppModel) Init() tea.Cmd {
	return tea.Batch(
		tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return TickMsg(t)
		}),
		tea.EnterAltScreen,
	)
}

func (m *AppModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		return m, nil
	case tea.KeyMsg:
		return m.handleKeyPress(msg)
	case TickMsg:
		m.timer.Tick()
		return m, tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return TickMsg(t)
		})
	}
	return m, nil
}

func (m *AppModel) handleKeyPress(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "q", "ctrl+c":
		return m, tea.Sequence(tea.ExitAltScreen, tea.Quit)
	case " ":
		// TODO
	case "r":
		// TODO
	case "b":
		// TODO
	case "n":
		// TODO
	}
	return m, nil
}

func (m *AppModel) View() string {
	return view.Render(view.Data{
		Width:       m.width,
		Height:      m.height,
		SessionType: view.WorkSessionType,
		TimerState:  m.timer.State,
		CurrentTime: m.timer.Current,
	})
}
