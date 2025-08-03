// Package cache
package cache

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/mt-shihab26/termodoro/pkg/enums"
	"github.com/mt-shihab26/termodoro/pkg/utils"
)

type Cache struct {
	SessionType     enums.SessionType `json:"session_type"`
	SessionCount    int               `json:"session_count"`
	TimerCurrent    int               `json:"timer_current"`
	SessionLastDate string            `json:"session_last_date"`
}

type PCache struct {
	SessionType     *enums.SessionType `json:"session_type,omitempty"`
	SessionCount    *int               `json:"session_count,omitempty"`
	TimerCurrent    *int               `json:"timer_current,omitempty"`
	SessionLastDate *string            `json:"session_last_date,omitempty"`
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
	if partial.SessionLastDate != nil {
		existingCache.SessionLastDate = *partial.SessionLastDate
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

func ensureCacheDir() error {
	cacheFullDir, err := getCacheDirName()
	if err != nil {
		return fmt.Errorf("failed to get cache directory name: %w", err)
	}
	return os.MkdirAll(cacheFullDir, 0755)
}

func getCacheFilePath() (string, error) {
	cacheFullDir, err := getCacheDirName()
	if err != nil {
		return "", fmt.Errorf("failed to get cache directory name: %w", err)
	}
	fileName := getTodayFileName()
	return filepath.Join(cacheFullDir, fileName), nil
}

func getCacheDirName() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("failed to get user home directory: %w", err)
	}
	cacheFullDir := filepath.Join(homeDir, ".local", "state", "termodoro")
	return cacheFullDir, nil
}

func getTodayFileName() string {
	year, month, day := time.Now().Date()
	return fmt.Sprintf("%04d-%02d-%02d.json", year, int(month), day)
}
