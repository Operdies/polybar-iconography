package main

import (
	"fmt"
	"time"
)

func main() {
	events := []string{
		"node_add",
		"node_remove",
		"node_focus",
		"node_flag",
		"desktop_focus",
	}
	source := Subscribe(events)
	AddHeartbeat(source, time.Second*2)
	for {
		evt := <-source
		fmt.Println(evt)
		wmState := GetWmState()
		fmt.Println(wmState.Monitors[0].Desktops[0])
	}
}
