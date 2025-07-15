package models

import (
	"fmt"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
	"github.com/mt-shihab26/termodoro/config"
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
		if m.timer.IsFinished() {
			m.completeCurrentSession()
			m.startNextSession()
		}
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
		m.timer.Toggle()
	case "r":
		m.timer.Reset()
	case "b":
		m.startBreakSession()
	case "n":
		m.completeCurrentSession()
		m.startNextSession()
	}
	return m, nil
}

func (m *AppModel) View() string {
	if m.width == 0 || m.height == 0 {
		return "Loading..."
	}

	// Format time as MM:SS
	minutes := m.timer.Current / 60
	seconds := m.timer.Current % 60
	timeStr := fmt.Sprintf("%02d:%02d", minutes, seconds)

	bigText := figure.NewFigure(timeStr, "big", true).String()

	// Session info
	sessionInfo := m.getSessionInfo()
	sessionStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("33")).
		Bold(true).
		MarginBottom(1)

	contentStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(m.getTimerColor()).
		Padding(1, 2).
		Align(lipgloss.Center)

	instructionsStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("240")).
		MarginTop(2)

	sessionInfoRendered := sessionStyle.Render(sessionInfo)
	content := contentStyle.Render(bigText)
	instructions := instructionsStyle.Render(m.getInstructions())

	combined := lipgloss.JoinVertical(
		lipgloss.Center,
		sessionInfoRendered,
		content,
		instructions,
	)

	return lipgloss.NewStyle().
		Width(m.width).
		Height(m.height).
		Align(lipgloss.Center, lipgloss.Center).
		Render(combined)
}

func (m *AppModel) getTimerColor() lipgloss.Color {
	switch m.timer.State {
	case Running:
		return m.getSessionColor()
	case Paused:
		return lipgloss.Color("208") // Orange
	default:
		return lipgloss.Color("240") // Gray
	}
}

func (m *AppModel) getSessionColor() lipgloss.Color {
	currentType := m.getCurrentSessionType()
	switch currentType {
	case Work:
		return lipgloss.Color("86") // Green
	case ShortBreak:
		return lipgloss.Color("39") // Blue
	case LongBreak:
		return lipgloss.Color("129") // Purple
	default:
		return lipgloss.Color("240") // Gray
	}
}

func (m *AppModel) getSessionInfo() string {
	currentType := m.getCurrentSessionType()
	completedWork := m.sessionManager.getCompletedWorkSessions()

	var sessionName string
	switch currentType {
	case Work:
		sessionName = "Work"
	case ShortBreak:
		sessionName = "Break"
	case LongBreak:
		sessionName = "Long Break"
	}

	return fmt.Sprintf("%s | Completed Today: %d", sessionName, completedWork)
}

func (m *AppModel) getCurrentSessionType() SessionType {
	completedWorkSessions := m.sessionManager.getCompletedWorkSessions()

	// If we have completed work sessions and the last session was work, we're in a break
	if len(m.sessionManager.Sessions) > 0 && m.sessionManager.Sessions[len(m.sessionManager.Sessions)-1].Type == Work {
		if completedWorkSessions > 0 && completedWorkSessions%m.sessionManager.SessionsBeforeLongBreak == 0 {
			return LongBreak
		}
		return ShortBreak
	}

	return Work
}

func (m *AppModel) completeCurrentSession() {
	currentType := m.getCurrentSessionType()
	session := Session{
		Type:      currentType,
		Duration:  m.timer.Duration,
		Completed: m.timer.Duration - m.timer.Current,
	}

	m.sessionManager.Sessions = append(m.sessionManager.Sessions, session)
}

func (m *AppModel) startNextSession() {
	nextDuration := m.sessionManager.GetCurrentSessionDuration()
	m.timer = NewTimer(nextDuration)
	m.timer.Start() // Auto-start the next session
}

func (m *AppModel) startBreakSession() {
	// Force start a break session
	completedWork := m.sessionManager.getCompletedWorkSessions()
	var breakDuration int

	if completedWork > 0 && completedWork%m.sessionManager.SessionsBeforeLongBreak == 0 {
		breakDuration = m.sessionManager.LongBreakDuration
	} else {
		breakDuration = m.sessionManager.ShortBreakDuration
	}

	// Add current session as completed if it was running
	if m.timer.State == Running {
		m.completeCurrentSession()
	}

	m.timer = NewTimer(breakDuration)
	m.timer.Start()
}

func (m *AppModel) getInstructions() string {
	operations := map[bool]string{
		true:  "Pause",
		false: "Start",
	}

	instructionTexts := []string{
		fmt.Sprintf("'SPACE': %s timer", operations[m.timer.State == Running]),
		"'R'    : Reset current session",
		"'B'    : Start break manually",
		"'N'    : Skip to next session",
		"'Q'    : Quit application",
	}

	return strings.Join(instructionTexts, "\n")
}
