package main

import (
  "bufio"
  "fmt"
  "os/exec"
)

func main() {
  args := []string { 
    "subscribe", 
    "node_add", 
    "node_remove",
    "node_focus", 
    "node_flag",
    "desktop_focus",
  }

  cmd := exec.Command("bspc", args ...)
  r, _ := cmd.StdoutPipe()
  _ = cmd.Start()
  scanner := bufio.NewScanner(r)

  for scanner.Scan() {
    line := scanner.Text()
    fmt.Println(line)
  }
}
