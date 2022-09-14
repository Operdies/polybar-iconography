package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os/exec"
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
	Desktops         []Desktop
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

type Client struct {
	ClassName string
	Urgent    bool
	Shown     bool
}

type WindowManagerState struct {
	FocusedMonitorId int
	ClientsCount     int
	Monitors         []Monitor
	FocusHistory     []FocusHistoryItem
	StackingList     []int
}

func Subscribe(args []string) chan string {
	arguments := append([]string{"subscribe"}, args...)
	cmd := exec.Command("bspc", arguments...)
	r, _ := cmd.StdoutPipe()
	_ = cmd.Start()
	scanner := bufio.NewScanner(r)

	messages := make(chan string)
	go func() {
		for scanner.Scan() {
			messages <- scanner.Text()
		}
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