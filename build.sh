#!/bin/bash

set -e 

subcommand=$1

[[ ${RELEASE:=0} -gt 0 ]] && release_flag="--release" || release_flag="" 

shift

case "$subcommand" in
    prep) 
        cargo chef prepare $release_flag $@
    ;;
    cook)
        cargo chef cook $release_flag $@
    ;;
    build)
        cargo build $release_flag $@
    ;;
    production) 
        docker compose -f docker-compose.yml -f docker-compose.prod.yml build
    ;;
    dev)
        docker compose -f docker-compose.yml -f docker-compose.dev.yml build 
    ;;
    *)  
        echo "Not a command: $subcommand" && exit 1
    ;;
esac 

