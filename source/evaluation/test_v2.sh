#!/bin/bash

export LD_LIBRARY_PATH=cpp/libs

SENSOR_COUNT=$1
VEHICLE_COUNT=$2
SECONDS=$3
OUT_FILE="times_for_${VEHICLE_COUNT}x${SENSOR_COUNT}_${SECONDS}s_v0.txt"


for ((i=0; i < 1000; ++i)); do
    OUT_FILE="times_for_${VEHICLE_COUNT}x${SENSOR_COUNT}_${SECONDS}s_v${i}.txt"
    if [ ! -f "$OUT_FILE" ]; then
        break;
    fi
done

if [ -f "$OUT_FILE" ]; then
    echo "Cant find file to write to"
    exit 1
fi

echo "$(date) Writing to $OUT_FILE"
rm -f "$OUT_FILE"
#rust/mecview_server --log=info &

CLIENT_ID=""

for ((i=0; i < $SENSOR_COUNT; i++)); do
    cpp/Client --type sensor -l info --nofile --noconsole -i 650 -d 100  &
done

sleep 1

for ((i=0; i < $VEHICLE_COUNT; i++)); do
    cpp/Client --type vehicle --enter 5 --exit "$SECONDS" --nofile | grep -Eo ' [0-9]+ ms' | cut -d\  -f2 >> "$OUT_FILE" &
    CLIENT_ID=$!
done

wait $CLIENT_ID
echo "$(date) shutdown"
sleep 2
jobs -p | xargs kill
wait
