// Package config
package config

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/mt-shihab26/termodoro/pkg/disk"
)

const (
	configDir      = ".config"
	appConfigDir   = "termodoro"
	configFileName = "config.json"
)

type Config struct {
	WorkSessionDuration      int `json:"work_session_duration"`
	BreakSessionDuration     int `json:"break_session_duration"`
	LongBreakSessionDuration int `json:"long_break_session_duration"`
}

var defaultConfig = Config{
	WorkSessionDuration:      25,
	BreakSessionDuration:     5,
	LongBreakSessionDuration: 15,
}

func mergeDefaults(config *Config) {
	if config.WorkSessionDuration == 0 {
		config.WorkSessionDuration = defaultConfig.WorkSessionDuration
	}
	if config.BreakSessionDuration == 0 {
		config.BreakSessionDuration = defaultConfig.BreakSessionDuration
	}
	if config.LongBreakSessionDuration == 0 {
		config.LongBreakSessionDuration = defaultConfig.LongBreakSessionDuration
	}
}

func Load() Config {
	configFilePath, err := getConfigFilePath()
	if err != nil {
		return defaultConfig
	}
	err = disk.GetIsFileExist(configFilePath)
	if err != nil {
		return defaultConfig
	}
	data, err := os.ReadFile(configFilePath)
	if err != nil {
		return defaultConfig
	}
	var config Config
	if err := json.Unmarshal(data, &config); err != nil {
		return defaultConfig
	}
	mergeDefaults(&config)
	return config
}

func getConfigFilePath() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("failed to get user home directory: %w", err)
	}
	configFullDir := filepath.Join(homeDir, configDir, appConfigDir)
	return filepath.Join(configFullDir, configFileName), nil
}
