package iconography

import (
	"strings"

	"github.com/operdies/polybar-iconography/pkg/bspc"
)

type col = func(a func() string, c string) string

type colorizer struct {
	Foreground col
	Background col
	Accent     col
}

var SETTINGS map[string]string
var ICONS map[string]string

func init() {
	SETTINGS = make(map[string]string)
	SETTINGS["WS_SEPARATOR"] = "┊"
	SETTINGS["FOCUSED_FOREGROUND"] = "#fff"
	SETTINGS["FOCUSED_BACKGROUND"] = "#0000"
	SETTINGS["FOCUSED_ACCENT"] = "#ac21c4"
	SETTINGS["URGENT_BACKGROUND"] = "#a22"
	SETTINGS["ACCENT_MODE"] = "under"

	ICONS = make(map[string]string)
	ICONS["default"] = ""
	ICONS["vim"] = ""
	ICONS["firefox"] = " "
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
