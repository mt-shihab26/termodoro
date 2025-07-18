// Package version
package version

import (
	"fmt"
	"runtime"
)

var (
	Version = "dev"
	Commit  = "unknown"
	Date    = "unknown"
)

func Run(args []string) error {
	arch := runtime.GOARCH
	if arch == "amd64" {
		arch = "x86_64"
	}
	fmt.Printf("termodoro version %s %s/%s\n", Version, runtime.GOOS, arch)
	if Commit != "unknown" {
		fmt.Printf("commit: %s\n", Commit)
	}
	if Date != "unknown" {
		fmt.Printf("built: %s\n", Date)
	}
	return nil
}
