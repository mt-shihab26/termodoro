package main

import (
	"github.com/mt-shihab26/termodoro/commands/help"
	"github.com/mt-shihab26/termodoro/commands/root"
	"github.com/mt-shihab26/termodoro/commands/version2"
	"github.com/mt-shihab26/termodoro/pkg/commands"
)

var (
	version = "dev"
	commit  = "unknown"
	date    = "unknown"
)

func main() {
	setVersion(version, commit, date)

	commands.Add(":", root.Run)
	commands.Add("help", help.Run)
	commands.Add("version", version2.Run)
}

func setVersion(ver, commit, date string) {
	version2.Version = ver
	version2.Commit = commit
	version2.Date = date
}
