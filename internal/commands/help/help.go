// Package help
package help

import "fmt"

func Run(args []string) error {
	fmt.Println("Termodoro - A terminal-based Pomodoro timer")
	fmt.Println("")
	fmt.Println("USAGE:")
	fmt.Println("    termodoro [COMMAND/FLAGS] ")
	fmt.Println("")
	fmt.Println("Run 'termodoro' without any command to start the TUI application.")
	fmt.Println("")
	fmt.Println("COMMANDS:")
	fmt.Println("    version    Show version information")
	fmt.Println("    help       Show this help message")
	fmt.Println("")
	fmt.Println("FLAGS:")
	fmt.Println("    -v, --version    Show version information")
	fmt.Println("    -h, --help       Show this help message")
	fmt.Println("")
	fmt.Println("CONTROLS (when running):")
	fmt.Println("    SPACE    Start/Pause timer")
	fmt.Println("    R        Reset current session")
	fmt.Println("    B        Start break manually")
	fmt.Println("    Q        Quit application")
	fmt.Println("")
	fmt.Println("For more information, visit: https://github.com/mt-shihab26/termodoro")
	return nil
}
