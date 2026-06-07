#!/bin/bash

PORT="/dev/pts/10"

echo "UART simulator started on $PORT"

i=0

while true; do
    echo "Current time: $(date +%T) | counter=$i" > $PORT
    echo "debug: sent packet $i"
    i=$((i+1))
    sleep 1
done
