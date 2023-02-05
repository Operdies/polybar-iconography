package main

import (
	_ "embed"
	"flag"
	"fmt"
	"log"
	"net"
	"os"
	"os/signal"
	"path/filepath"
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

//go:embed config.yml
var defaultConfig []byte

var configFile string

func exists(path string) bool {
	_, ok := os.Stat(path)
	return ok == nil
}

func readAndParse(configFile string) (*iconography.Config, error) {
	content, err := os.ReadFile(configFile)
	if err != nil {
		return nil, err
	}
	return iconography.ParseConfig(content)
}
func getConfig() (*iconography.Config, error) {
	// 1. -config-file CLI argument
	if configFile != "" {
		return readAndParse(configFile)
	}
	// 2. config.yml next to iconography executable
	if len(os.Args) > 0 {
		executable := os.Args[0]
		path := filepath.Dir(executable)
		testpath := filepath.Join(path, "config.yml")
		if exists(testpath) {
			return readAndParse(testpath)
		}
	}
	if os.Getenv("HOME") != "" {
		// 3. $HOME/.config/iconography/config.yml
		testpath := os.ExpandEnv("$HOME/.config/iconography/config.yml")
		if exists(testpath) {
			return readAndParse(testpath)
		}
		// 4. $HOME/.iconography.yml
		testpath = os.ExpandEnv("$HOME/.iconograhpy.yml")
		if exists(testpath) {
			return readAndParse(testpath)
		}
	}
	// 5. The default config
	return iconography.ParseConfig(defaultConfig)
}

func main() {
	log.SetFlags(0)
	dumpConfig := flag.Bool("dump-config", false, "Dump the default config to stdout and exit")
	flag.StringVar(&configFile, "config-file", "", "The path to a config file")
	ipc := flag.Bool("ipc", false, "Interpret the remaining arguments as IPC arguments, and exit after communicating with main process.")
	flag.Parse()

	if *dumpConfig {
		_, err := iconography.ParseConfig(defaultConfig)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Print(string(defaultConfig))
		return
	}

	if *ipc {
		command := flag.Args()
		str := strings.Join(command, " ")
		write_socket(str)
		return
	}

	cfg, err := getConfig()
	if err != nil {
		log.Fatal(err)
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

	drawer := iconography.CreateDrawer(cfg)
	for {
		wm := bspc.GetWmState()
		drawing := drawer.Draw(wm)
		fmt.Println(drawing)
		_, ok := <-source
		if !ok {
			break
		}
	}
	fmt.Printf("Exit\n")
}
