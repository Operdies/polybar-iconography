#!/bin/bash 

cd ./iconography/
while true; do 
  go run . &
  inotifywait -q -e close_write -r .
  pkill iconography
done

