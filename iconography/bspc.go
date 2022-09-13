package main

import (
	"bufio"
	"os/exec"
)

func Subscribe(args []string) func() string {
	arguments := append([]string{"subscribe"}, args...)
	cmd := exec.Command("bspc", arguments...)
	r, _ := cmd.StdoutPipe()
	_ = cmd.Start()
	scanner := bufio.NewScanner(r)

	return func() string {
    scanner.Scan()
    return scanner.Text()
	}
}
