package iconography

import (
	"strings"

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

func getClientNodes(node *bspc.Node) []*bspc.Node {
	var result []*bspc.Node

	if node == nil {
		return result
	}
	if node.Client.ClassName != "" {
		result = append(result, node)
	}
	result = append(result, getClientNodes(node.FirstChild)...)
	return append(result, getClientNodes(node.SecondChild)...)
}

func Draw(wm bspc.WindowManagerState) string {
	var sb strings.Builder
	for _, mon := range wm.Monitors {
		monFocused := wm.FocusedMonitorId == mon.Id
		for _, workspace := range mon.Desktops {
			wsFocused := monFocused && mon.FocusedDesktopId == workspace.Id
      nodes := getClientNodes(&workspace.Root)

      if wsFocused || len(nodes) > 0 {
        sb.WriteString("This one")
      }
			for _, node := range nodes {
				client := node.Client
				clientFocused := wsFocused && workspace.FocusedNodeId == node.Id
				if !clientFocused {
					sb.WriteString(client.ClassName + " - ")
				}
			}
		}

	}
	return sb.String()
}
