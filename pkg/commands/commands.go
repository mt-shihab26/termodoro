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

func Run() {
	args := os.Args[1:]

	if len(args) == 0 {
		cmd := registry[":"]
		cmd.handler()(args)
		return
	}

	fmt.Println(args)
}
