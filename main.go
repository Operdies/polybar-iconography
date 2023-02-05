package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"os"
	"os/signal"
	"strings"
	"syscall"
	"time"

	"github.com/operdies/polybar-iconography/pkg/bspc"
	"github.com/operdies/polybar-iconography/pkg/iconography"
)

const sockfile = "/tmp/iconography.sock"

func handle_ipc(command string) {
  command = strings.TrimSpace(command)
  words := strings.Split(command, " ")
  for i, w := range words {
    fmt.Printf("%v) '%v'\n", i, w)
  }
}

func socker() {
	socket, err := net.Listen("unix", sockfile)

	if err != nil {
		log.Fatal(err)
	}

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGINT, syscall.SIGKILL)
	fmt.Printf("Setting handler\n")
	go func() {
		sig := <-c
		fmt.Printf("%v Interrupt received\n", sig)
		os.Remove(sockfile)
		os.Exit(1)
	}()

	for {
		conn, err := socket.Accept()
		if err != nil {
			log.Fatal(err)
		}

		go func(conn net.Conn) {
			defer conn.Close()
			buf := make([]byte, 100)
			conn.SetReadDeadline(time.Now().Add(time.Millisecond * 10))
      n, _ := conn.Read(buf)
      handle_ipc(string(buf[:n]))
		}(conn)
	}

}

func write_socket(message string) {
  socket, err := net.Dial("unix", sockfile)
  defer socket.Close()
  if err != nil {
    log.Fatal(err)
  }
  socket.Write([]byte(message))
}

func main() {
  ipc := flag.Bool("ipc", false, "If set, interpret the remaining arguments as IPC arguments, and exit after communicating with main process.")
	flag.Parse()
  if *ipc {
    command := flag.Args()
    str := strings.Join(command, " ")
    write_socket(str)
    return
  }

	events := []string{
		"node_add",
		"node_remove",
		"node_focus",
		"node_flag",
		"desktop_focus",
	}

	source := bspc.Subscribe(events, 0)

	go socker()

	for {
		wm := bspc.GetWmState()
		drawing := iconography.Draw(wm)
		fmt.Println(drawing)
		_, ok := <-source
		if !ok {
			break
		}
	}
	fmt.Printf("Exit\n")
}
