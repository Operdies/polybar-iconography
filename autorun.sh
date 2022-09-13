#!/bin/bash 

while true; do 
  go build ./src/Iconography.go
  ./Iconography &
  id=$!
  inotifywait -q -e close_write -r ./src
  pkill Iconography
done

pkill Iconography
