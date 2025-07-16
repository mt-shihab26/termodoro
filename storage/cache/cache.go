// Package cache
package cache

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

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

func New(sessionType view.SessionType, sessionCount int) *Cache {
	return &Cache{
		SessionType:  sessionType,
		SessionCount: sessionCount,
	}
}

func Load() (*Cache, error) {
	cacheFilePath, err := getCacheFilePath()
	if err != nil {
		return nil, err
	}
	err = getIsFileExist(cacheFilePath)
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

func Save(cache *Cache) error {
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

func getIsFileExist(filePath string) error {
	_, err := os.Stat(filePath)
	if err != nil {
		return err
	}
	return nil
}

func ensureCacheDir() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return fmt.Errorf("failed to get user home directory: %w", err)
	}
	cacheFullDir := filepath.Join(homeDir, cacheDir)
	return os.MkdirAll(cacheFullDir, 0755)
}

// func ClearCache() error {
// 	cacheFilePath, err := getCacheFilePath()
// 	if err != nil {
// 		return err
// 	}
//
// 	if err := os.Remove(cacheFilePath); err != nil && !os.IsNotExist(err) {
// 		return fmt.Errorf("failed to remove cache file: %w", err)
// 	}
//
// 	return nil
// }
//
// func (session *Session) Reset() {
// 	session.State = view.WorkSessionType
// 	session.Count = 0
//
// 	if err := ClearCache(); err != nil {
// 		fmt.Printf("Warning: failed to clear cache: %v\n", err)
// 	}
//
// 	if err := session.saveToCache(); err != nil {
// 		fmt.Printf("Warning: failed to save reset session to cache: %v\n", err)
// 	}
// }
