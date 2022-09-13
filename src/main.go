package main

import (
  "bufio"
  "fmt"
  "os/exec"
)

func main() {
  cmd := exec.Command("bspc", "subscribe", "all")
  r, _ := cmd.StdoutPipe()
  _ = cmd.Start()
  scanner := bufio.NewScanner(r)

  for scanner.Scan() {
    line := scanner.Text()
    fmt.Println(line)
  }
}
