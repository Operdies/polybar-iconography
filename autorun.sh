#!/bin/bash 

while true; do 
  go run . &
  inotifywait -q -e close_write -r .
  pidof -x polybar-iconography | xargs kill 
done

