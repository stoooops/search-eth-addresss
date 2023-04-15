#!/bin/bash

POSITIONAL=()
CMD_ARGS=""

while [[ $# -gt 0 ]]
do
key="$1"
case $key in
    --threads)
    THREADS="$2"
    CMD_ARGS+="--threads ${THREADS} "
    shift
    shift
    ;;
    --jobs)
    JOBS="$2"
    CMD_ARGS+="--jobs ${JOBS} "
    shift
    shift
    ;;
    --each)
    EACH="$2"
    CMD_ARGS+="--each ${EACH} "
    shift
    shift
    ;;
    *)
    POSITIONAL+=("$1")
    CMD_ARGS+="$1 "
    shift
    ;;
esac
done

export CMD_ARGS

# Set the desired directory path and ownership
LOG_DIR="/var/log/vanitygen"
LOG_PERMS="700"

# Check if the log directory exists, and create it with sudo if necessary
if [ ! -d "$LOG_DIR" ]; then
  echo "Creating log directory $LOG_DIR ..."
  sudo mkdir -p "$LOG_DIR"
  sudo chmod "$LOG_PERMS" "$LOG_DIR"
else
  echo "Log directory $LOG_DIR already exists, skipping creation."
fi

docker-compose up --force-recreate
