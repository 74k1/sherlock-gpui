#!/bin/bash

# Pomodoro timer integration for Sherlock.
# This script is called by Sherlock at the end of each timer. It sends a
# desktop notification asking the user to start the next phase, then schedules
# a new Sherlock timer for that phase if confirmed. If the notification is dismissed,
# no new timer will be started.
#
# Each cycle consists of a focused work sprint followed by a short break.
# After every 4 sprints, a longer break is taken to recharge.
#
# Timings (configurable below):
#   Focus sprint  —  25 min
#   Short break   —   5 min
#   Long break    —  15 min  (every 4th sprint)

# Customizable Durations
WORK_DURATION="25m"
SHORT_BREAK="5m"
LONG_BREAK="15m"
LONG_BREAK_INTERVAL=4

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
SCRIPT_PATH="$SCRIPT_DIR/$(basename "$0")"
MODE=${1:-"pomodoro"}
ITERATION=${2:-1}

if [ "$MODE" = "break" ]; then
    TITLE="Working Period is Over"
    PROMPT="Start Break?"
    DURATION=$(( ITERATION % LONG_BREAK_INTERVAL == 0 ? 1 : 0 )) 
    DURATION=$([ "$DURATION" -eq 1 ] && echo "$LONG_BREAK" || echo "$SHORT_BREAK")
    NEXT_MODE="pomodoro"
    NEXT_ITERATION=$ITERATION
else
    TITLE="Break is Over"
    PROMPT="Start New Focus?"
    DURATION="$WORK_DURATION"
    NEXT_MODE="break"
    NEXT_ITERATION=$(( ITERATION + 1 ))
fi

ACTION=$(notify-send "$TITLE" "$PROMPT" --action="yes=Yes" --wait)
if [ "$ACTION" = "yes" ]; then
    sherlock new-timer "$DURATION" "$SCRIPT_PATH $NEXT_MODE $NEXT_ITERATION" >/dev/null 2>&1 &
fi

