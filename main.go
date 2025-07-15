package main

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mt-shihab26/termodoro/config"
	"github.com/mt-shihab26/termodoro/models"
)

func main() {
	c := config.New()

	m := models.NewAppModel(c)

	p := tea.NewProgram(
		m,
		tea.WithAltScreen(),
		tea.WithMouseCellMotion(),
	)

	if _, err := p.Run(); err != nil {
		fmt.Printf("Error: %v", err)
	}
}
