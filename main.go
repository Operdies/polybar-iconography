package main

import (
	"fmt"
	"log"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/operdies/polybar-iconography/pkg/bspc"
	// "github.com/operdies/polybar-iconography/iconography"
)

const sockfile = "/tmp/iconography.sock"

func socker(){
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
    }(conn)
  }


}

func main() {
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
		e, ok := <-source
		if !ok {
			break
		}
    fmt.Printf("%v\n", e)
		// wm := bspc.GetWmState()
		// drawing := iconography.Draw(wm)
		// fmt.Println(drawing)
	}
}
