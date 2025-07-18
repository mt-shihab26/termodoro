package main

import "github.com/mt-shihab26/termodoro/internal/commands/root"

var (
	version = "dev"
	commit  = "unknown"
	date    = "unknown"
)

func main() {
	root.SetVersion(version, commit, date)
	root.Execute()
}
