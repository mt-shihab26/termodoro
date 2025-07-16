// Package cache
package cache

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/mt-shihab26/termodoro/pkg/disk"
	"github.com/mt-shihab26/termodoro/view"
)

const (
	cacheDir      = ".termodoro"
	cacheFileName = "cache.json"
)

type Cache struct {
	SessionType  view.SessionType `json:"session_type"`
	SessionCount int              `json:"session_count"`
}

type PCache struct {
	SessionType  *view.SessionType `json:"session_type,omitempty"`
	SessionCount *int              `json:"session_count,omitempty"`
}

func Load() (*Cache, error) {
	cacheFilePath, err := getCacheFilePath()
	if err != nil {
		return nil, err
	}
	err = disk.GetIsFileExist(cacheFilePath)
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
	return filepath.Join(cacheFullDir, cacheFileName), nil
}

func ensureCacheDir() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return fmt.Errorf("failed to get user home directory: %w", err)
	}
	cacheFullDir := filepath.Join(homeDir, cacheDir)
	return os.MkdirAll(cacheFullDir, 0755)
}
