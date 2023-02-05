package bspc

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os/exec"
	"strconv"
	"time"
)

type FocusHistoryItem struct {
	MonitorId int
	DesktopId int
	NodeId    int
}

type Monitor struct {
	Name             string
	Id               int
	RandrId          int
	FocusedDesktopId int
	Desktops         []*Desktop
}

type Desktop struct {
	Name          string
	Id            int
	FocusedNodeId int
	Root          Node
}

type Node struct {
	Id          int
	Client      Client
	FirstChild  *Node
	SecondChild *Node
	Hidden      bool
	Sticky      bool
	Private     bool
	Locked      bool
	Marked      bool
	SplitType   string
	SplitRatio  float64
}

func (n Node) Focus() {
	cmd := exec.Command("bspc", "node", "-f", strconv.FormatInt(int64(n.Id), 10))
	cmd.Start()
}

type Client struct {
	ClassName string
	Urgent    bool
	Shown     bool
}

/* this client will be decorated with additional information when retrieved by GetAllClients */
type DecoratedClient struct {
	Client       Client
	Node         *Node
	Desktop      *Desktop
	Monitor      *Monitor
	Wm           *WindowManagerState
	DesktopIndex int
	Focused      bool
}

type WindowManagerState struct {
	FocusedMonitorId int
	ClientsCount     int
	Monitors         []*Monitor
	FocusHistory     []*FocusHistoryItem
	StackingList     []*int
}

func Subscribe(args []string, count int) chan string {
	arguments := append([]string{"subscribe"}, args...)
	if count > 0 {
		carr := []string{"-c", strconv.FormatInt(int64(count), 10)}
		arguments = append(arguments, carr...)
	}

	cmd := exec.Command("bspc", arguments...)
	r, _ := cmd.StdoutPipe()
	_ = cmd.Start()
	scanner := bufio.NewScanner(r)

	messages := make(chan string)
	go func() {
		for scanner.Scan() {
			messages <- scanner.Text()
		}
		close(messages)
	}()

	return messages
}

// Add a heartbeat to some channel
func AddHeartbeat(messages chan string, interval time.Duration) {
	go func() {
		for {
			time.Sleep(interval)
			messages <- "Heartbeat"
		}
	}()
}

func recursiveGetClients(n *Node) []DecoratedClient {
	clients := make([]DecoratedClient, 0)
	if n != nil && n.Client.ClassName != "" {
		clients = make([]DecoratedClient, 1)
		var c DecoratedClient
		c.Client = n.Client
		c.Node = n
		clients[0] = c
	}

	if n.FirstChild != nil {
		clients = append(clients, recursiveGetClients(n.FirstChild)...)
	}
	if n.SecondChild != nil {
		clients = append(clients, recursiveGetClients(n.SecondChild)...)
	}
	return clients
}

func GetAllClients(wm *WindowManagerState) []DecoratedClient {
	clients := make([]DecoratedClient, wm.ClientsCount)
	cnt := 0
	for _, mon := range wm.Monitors {
		monFocused := mon.Id == wm.FocusedMonitorId
		for idx, desktop := range mon.Desktops {
			desktopFocused := monFocused && desktop.Id == mon.FocusedDesktopId
			subclients := recursiveGetClients(&desktop.Root)
			for _, s := range subclients {
				s.Desktop = desktop
				s.Monitor = mon
				s.Wm = wm
        s.DesktopIndex = idx
				s.Focused = desktopFocused && desktop.FocusedNodeId == s.Node.Id
				clients[cnt] = s
				cnt = cnt + 1
			}
		}
	}

	return clients
}

func GetWmState() WindowManagerState {
	var wmState WindowManagerState
	out, _ := exec.Command("bspc", "wm", "-d").Output()
	// s := string(out)
	err := json.Unmarshal(out, &wmState)

	if err != nil {
		fmt.Println(err)
	}

	return wmState
}
