package models

import (
	"fmt"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
)

// TickMsg represents a tick message
type TickMsg time.Time

// AppModel represents the main application model
type AppModel struct {
	timer  *Timer
	width  int
	height int
}

// NewAppModel creates a new application model
func NewAppModel(timerDuration int) *AppModel {
	return &AppModel{
		timer: NewTimer(timerDuration),
	}
}

// Init initializes the application
func (m *AppModel) Init() tea.Cmd {
	return tea.Batch(
		tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return TickMsg(t)
		}),
		tea.EnterAltScreen,
	)
}

// Update handles messages and updates the model
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

// handleKeyPress handles keyboard input
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

// View renders the application
func (m *AppModel) View() string {
	if m.width == 0 || m.height == 0 {
		return "Loading..."
	}

	// Generate ASCII art for the counter
	counterStr := fmt.Sprintf("%d", m.timer.Current)
	bigText := figure.NewFigure(counterStr, "big", true).String()

	// Style for the counter value
	contentStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(m.getTimerColor()).
		Padding(1, 2).
		Align(lipgloss.Center)

	// Style for instructions
	instructionsStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("240")).
		MarginTop(2)

	// Render the counter with ASCII art
	content := contentStyle.Render(bigText)

	// Render instructions
	instructions := instructionsStyle.Render(m.getInstructions())

	// Add status indicator
	statusStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("208")).
		MarginTop(1).
		Bold(true)
	
	status := statusStyle.Render(m.getStatusText())

	// Combine all elements vertically
	combined := lipgloss.JoinVertical(
		lipgloss.Center, 
		content, 
		status,
		instructions,
	)

	// Center everything in the terminal
	return lipgloss.NewStyle().
		Width(m.width).
		Height(m.height).
		Align(lipgloss.Center, lipgloss.Center).
		Render(combined)
}

// getTimerColor returns the appropriate color based on timer state
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

// getStatusText returns the current status text
func (m *AppModel) getStatusText() string {
	switch m.timer.State {
	case Running:
		return "⏱️  RUNNING"
	case Paused:
		return "⏸️  PAUSED"
	default:
		return "⏹️  STOPPED"
	}
}

// getInstructions returns the instruction text
func (m *AppModel) getInstructions() string {
	instructionTexts := []string{
		"'SPACE': Start/Pause timer",
		"'R': Reset current session",
		"'B': Start break manually",
		"'Q': Quit application",
	}
	return strings.Join(instructionTexts, "\n")
}
