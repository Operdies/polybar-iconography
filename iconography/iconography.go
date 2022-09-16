package iconography

import (
	"fmt"
	"github.com/operdies/polybar-iconography/bspc"
)

type col = func(a func() string, c string) string

type colorizer struct {
	Foreground col
	Background col
	Accent     col
}

func getColorizer() colorizer {
	var colorizer colorizer
	foregrounds := 0
	fg := func(a func() string, c string) string {
		foregrounds += 1
		str := a()
		fmt := "{F" + c + "}" + str
		foregrounds -= 1
		if foregrounds == 0 {
			fmt += "{F-}"
		}
		return fmt
	}
	colorizer.Foreground = fg
	return colorizer
}

func Draw() {
	events := []string{
		"node_add",
		"node_remove",
		"node_focus",
		"node_flag",
		"desktop_focus",
	}

	source := bspc.Subscribe(events, 2)

	for {
		evt, ok := <-source
		if !ok {
			break
		}
		fmt.Println(evt)
	}
}
