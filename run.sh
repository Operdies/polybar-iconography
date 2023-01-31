#!/bin/bash 

gwatch -mode kill -command 'rm /tmp/iconography.sock; go build . && ./polybar-iconography' main.go ./pkg
