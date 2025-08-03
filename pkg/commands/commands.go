// Package commands
package commands

import (
	"fmt"
	"os"
)

type Func func(args []string) error

type command struct {
	handler func() Func
}

var registry = make(map[string]command)

func Add(name string, handler func() Func) {
	registry[name] = command{handler: handler}
}

func Run() error {
	if len(os.Args[1:]) == 0 {
		cmd, ok := registry[":"]
		if !ok {
			return fmt.Errorf("default command ':' not found")
		}
		return cmd.handler()([]string{})
	}

	firstSubcommand := os.Args[1]
	cmd, ok := registry[firstSubcommand]
	if !ok {
		return fmt.Errorf("unknown command: %s", firstSubcommand)
	}
	return cmd.handler()(os.Args[2:])
}
