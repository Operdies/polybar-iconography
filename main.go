package main

import (
	"fmt"

	"github.com/operdies/polybar-iconography/bspc"
	"github.com/operdies/polybar-iconography/iconography"
)

func main() {
	events := []string{
		"node_add",
		"node_remove",
		"node_focus",
		"node_flag",
		"desktop_focus",
	}

	source := bspc.Subscribe(events, 0)

	for {
		_, ok := <-source
		if !ok {
			break
		}
		wm := bspc.GetWmState()
		drawing := iconography.Draw(wm)
		fmt.Println(drawing)
	}
}
