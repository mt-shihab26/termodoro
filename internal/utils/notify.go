package utils

import (
	"github.com/gen2brain/beeep"
)

func Notify(title, message, iconPath string) error {
	return beeep.Notify(title, message, iconPath)
}

func NotifyWithSound(title, message, iconPath string) error {
	err := Notify(title, message, iconPath)
	if err != nil {
		return err
	}
	return beeep.Beep(beeep.DefaultFreq, beeep.DefaultDuration)
}
