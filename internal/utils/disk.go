package utils

import "os"

func GetIsFileExist(filePath string) error {
	_, err := os.Stat(filePath)
	if err != nil {
		return err
	}
	return nil
}
