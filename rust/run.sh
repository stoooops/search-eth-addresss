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
docker-compose up --force-recreate
