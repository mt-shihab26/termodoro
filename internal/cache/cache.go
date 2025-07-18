// Package cache
package cache

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/mt-shihab26/termodoro/internal/ui"
	"github.com/mt-shihab26/termodoro/internal/utils"
)

const (
	cacheDir = ".termodoro"
)

type Cache struct {
	SessionType  ui.SessionType `json:"session_type"`
	SessionCount int            `json:"session_count"`
	TimerCurrent int            `json:"timer_current"`
}

type PCache struct {
	SessionType  *ui.SessionType `json:"session_type,omitempty"`
	SessionCount *int            `json:"session_count,omitempty"`
	TimerCurrent *int            `json:"timer_current,omitempty"`
}

func Load() (*Cache, error) {
	cacheFilePath, err := getCacheFilePath()
	if err != nil {
		return nil, err
	}
	err = utils.GetIsFileExist(cacheFilePath)
	if err != nil {
		return nil, err
	}
	data, err := os.ReadFile(cacheFilePath)
	if err != nil {
		return nil, fmt.Errorf("failed to read cache file: %w", err)
	}
	var cache Cache
	if err := json.Unmarshal(data, &cache); err != nil {
		return nil, fmt.Errorf("failed to unmarshal cache data: %w", err)
	}
	return &cache, nil
}

func LoadTimerCurrent() (int, error) {
	che, err := Load()
	if err != nil {
		return 0, err
	}
	cached := che.TimerCurrent
	if cached == 0 {
		return 0, errors.New("timer_current is zero value")
	}
	zero := 0
	Save(&PCache{TimerCurrent: &zero})
	return cached, nil
}

func Save(partial *PCache) error {
	existingCache, err := Load()
	if err != nil {
		existingCache = &Cache{}
	}
	if partial.SessionType != nil {
		existingCache.SessionType = *partial.SessionType
	}
	if partial.SessionCount != nil {
		existingCache.SessionCount = *partial.SessionCount
	}
	if partial.TimerCurrent != nil {
		existingCache.TimerCurrent = *partial.TimerCurrent
	}
	return save(existingCache)
}

func save(cache *Cache) error {
	err := ensureCacheDir()
	if err != nil {
		return fmt.Errorf("failed to create cache directory: %w", err)
	}
	cacheFilePath, err := getCacheFilePath()
	if err != nil {
		return err
	}
	data, err := json.Marshal(cache)
	if err != nil {
		return fmt.Errorf("failed to marshal cache data: %w", err)
	}
	err = os.WriteFile(cacheFilePath, data, 0644)
	if err != nil {
		return fmt.Errorf("failed to write cache file: %w", err)
	}
	return nil
}

func getCacheFilePath() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("failed to get user home directory: %w", err)
	}
	cacheFullDir := filepath.Join(homeDir, cacheDir)
	fileName := getTodayFileName()
	return filepath.Join(cacheFullDir, fileName), nil
}

func getTodayFileName() string {
	year, month, day := time.Now().Date()
	return strings.ToLower(fmt.Sprintf("%v-%v-%v.json", year, month, day))
}

func ensureCacheDir() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return fmt.Errorf("failed to get user home directory: %w", err)
	}
	cacheFullDir := filepath.Join(homeDir, cacheDir)
	return os.MkdirAll(cacheFullDir, 0755)
}
