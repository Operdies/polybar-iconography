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

func getClients(node bspc.Node) []bspc.Client {
	var result []bspc.Client

	if node.Client.ClassName != "" {
		result = append(result, node.Client)
	}

	return result
}

func Draw(wm bspc.WindowManagerState) string {
	var sb strings.Builder
	for _, mon := range wm.Monitors {
		for _, workspace := range mon.Desktops {
			for _, client := range getClients(workspace.Root) {
				sb.WriteString(client.ClassName)
			}
		}

	}
	return sb.String()
}
