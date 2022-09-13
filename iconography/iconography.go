package main

import "fmt"

func main() {
	events := []string{
		"node_add",
		"node_remove",
		"node_focus",
		"node_flag",
		"desktop_focus",
	}
  source := Subscribe(events)
  for {
    evt := source()
    fmt.Println(evt)
  }
}
