package models

import (
	"fmt"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
)

type TickMsg time.Time

type AppModel struct {
	timer  *Timer
	width  int
	height int
}

func NewAppModel(timerDuration int) *AppModel {
	return &AppModel{
		timer: NewTimer(timerDuration),
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
		m.timer.Toggle()
	case "r":
		m.timer.Reset()
	case "b":
		// Start break manually (placeholder for future implementation)
		// This could be extended to switch to break timer
	}
	return m, nil
}

func (m *AppModel) View() string {
	if m.width == 0 || m.height == 0 {
		return "Loading..."
	}

	counterStr := fmt.Sprintf("%d", m.timer.Current)
	bigText := figure.NewFigure(counterStr, "big", true).String()

	contentStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(m.getTimerColor()).
		Padding(1, 2).
		Align(lipgloss.Center)

	instructionsStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("240")).
		MarginTop(2)

	content := contentStyle.Render(bigText)

	instructions := instructionsStyle.Render(m.getInstructions())

	statusStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("208")).
		MarginTop(1).
		Bold(true)

	status := statusStyle.Render(m.getStatusText())

	combined := lipgloss.JoinVertical(
		lipgloss.Center,
		content,
		status,
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
		return lipgloss.Color("86") // Green
	case Paused:
		return lipgloss.Color("208") // Orange
	default:
		return lipgloss.Color("240") // Gray
	}
}

func (m *AppModel) getStatusText() string {
	switch m.timer.State {
	case Running:
		return "RUNNING"
	case Paused:
		return "PAUSED"
	default:
		return "STOPPED"
	}
}

func (m *AppModel) getInstructions() string {
	instructionTexts := []string{
		"'SPACE': Start/Pause timer",
		"'R': Reset current session",
		"'B': Start break manually",
		"'Q': Quit application",
	}
	return strings.Join(instructionTexts, "\n")
}
