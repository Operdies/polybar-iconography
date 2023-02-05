package iconography

import (
	"log"
	"regexp"
	"strings"

	"github.com/operdies/polybar-iconography/pkg/bspc"
	"gopkg.in/yaml.v3"
)

type Colors struct {
	Foreground string
	Background string
	Accent     string
}

type Separators struct {
	Before  string
	After   string
	Between string
}

type Config struct {
  Monitors struct {
    Separators Separators
  }
	Desktops struct {
		Separators Separators
	}
	Colors struct {
		Accent_mode string
		Normal      Colors
		Focused     Colors
		Urgent      Colors
	}
	Icons struct {
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

type Drawer struct {
	config *Config
	icons  map[*regexp.Regexp]string
}

func CreateDrawer(config *Config) *Drawer {
	drawer := &Drawer{config: config, icons: make(map[*regexp.Regexp]string)}
	for _, m := range config.Icons.Mappings {
		pattern, err := regexp.Compile(m.Pattern)
		if err != nil {
			log.Fatalf("Error parsing expression '%v': %v", m.Pattern, err)
		} else {
			drawer.icons[pattern] = m.Icon
		}
	}
	return drawer
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

func (d *Drawer) getIcon(name string) string {
	for expr, icon := range d.icons {
		if expr.Match([]byte(name)) {
			return icon
		}
	}
	return name
}

var (
	subscripts   = []string{"₁", "₂", "₃", "₄", "₅", "₆", "₇", "₈", "₉", "₀"}
	superscripts = []string{"⁰", "¹", " ²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "ⁿ"}
)

func (d *Drawer) Draw(wm bspc.WindowManagerState) string {
	var sb strings.Builder
	for _, mon := range wm.Monitors {
		monFocused := wm.FocusedMonitorId == mon.Id
		for _, workspace := range mon.Desktops {
			wsFocused := monFocused && mon.FocusedDesktopId == workspace.Id
			nodes := getClientNodes(&workspace.Root)

			for _, node := range nodes {
				client := node.Client
				clientFocused := wsFocused && workspace.FocusedNodeId == node.Id
				icon := d.getIcon(client.ClassName)
				if !clientFocused {
					sb.WriteString(icon + " - ")
				} else {
					sb.WriteString("[ " + icon + " ]" + " - ")
				}
			}
		}

	}
	return sb.String()
}
