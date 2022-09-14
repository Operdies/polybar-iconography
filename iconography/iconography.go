package main

import (
	"fmt"
	"time"
)

type Col = func(a func() string, c string) string 

type Colorizer struct {
  Foreground Col 
  Background Col 
  Accent Col
}

func getColorizer() Colorizer {
  var colorizer Colorizer 
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
		// wmState := GetWmState()
		// for _, mon := range wmState.Monitors {
		// 	for _, desk := range mon.Desktops {
		// 	}
		// }
	}
}
