package iconography

import (
	"strings"

	"github.com/operdies/polybar-iconography/pkg/bspc"
	"gopkg.in/yaml.v3"
)

type Colors struct {
	Foreground string
	Background string
	Accent     string
}

type Config struct {
	Workspace_separators struct {
		Before  string
		After   string
		Between string
	}
	Colors struct {
		Accent_mode string
		Normal      Colors
		Focused     Colors
		Urgent      Colors
	}
	Icons struct {
		Fallback string
		Mappings []struct {
			Pattern string
			Icon    string
		}
	}
}

func ParseConfig(config []byte) (*Config, error) {
	cfg := &Config{}
	err := yaml.Unmarshal(config, &cfg)
	return cfg, err
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
