package main

import (
	"fmt"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
)

type model struct {
	counter int
	width   int
	height  int
}

type tickMsg time.Time

func (m model) Init() tea.Cmd {
	return tea.Batch(
		tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return tickMsg(t)
		}),
		tea.EnterAltScreen,
	)
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		return m, nil
	case tea.KeyMsg:
		switch msg.String() {
		case "q", "ctrl+c":
			return m, tea.Sequence(tea.ExitAltScreen, tea.Quit)
		}
	case tickMsg:
		m.counter++
		return m, tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return tickMsg(t)
		})
	}
	return m, nil
}

func (m model) View() string {
	if m.width == 0 || m.height == 0 {
		return "Loading..."
	}

	// Generate ASCII art for the counter using go-figure
	counterStr := fmt.Sprintf("%d", m.counter)
	bigText := figure.NewFigure(counterStr, "big", true).String()

	// Style for the counter value
	contentStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("86")).
		Padding(1, 2).
		Align(lipgloss.Center)

	// Style for instructions
	instructionsStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("240")).
		Align(lipgloss.Center).
		MarginTop(2)

	// Render the counter with ASCII art
	content := contentStyle.Render(bigText)

	instructionTexts := []string{
		"'SPACE': start/stop",
		"'q': quit",
	}

	// Render instructions
	instructions := instructionsStyle.Render(strings.Join(instructionTexts, "\n"))

	// Combine content and instructions vertically
	combined := lipgloss.JoinVertical(lipgloss.Center, content, instructions)

	// Center everything in the terminal
	return lipgloss.NewStyle().
		Width(m.width).
		Height(m.height).
		Align(lipgloss.Center, lipgloss.Center).
		Render(combined)
}

func main() {
	p := tea.NewProgram(
		model{},
		tea.WithAltScreen(),
		tea.WithMouseCellMotion(),
	)
	if _, err := p.Run(); err != nil {
		fmt.Printf("Error: %v", err)
	}
}
