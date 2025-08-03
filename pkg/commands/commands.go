// Package commands
package commands

type Func func(args []string) error

func Add(command string, handler func() Func) {

}
