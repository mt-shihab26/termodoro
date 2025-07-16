// Package version
package version

import "fmt"

var (
	Version = "dev"
	Commit  = "unknown"
	Date    = "unknown"
)

func Run(args []string) error {
	fmt.Printf("termodoro version %s\n", Version)
	if Commit != "unknown" {
		fmt.Printf("commit: %s\n", Commit)
	}
	if Date != "unknown" {
		fmt.Printf("built: %s\n", Date)
	}
	return nil
}
