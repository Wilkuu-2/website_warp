#!/bin/bash

 set -e 

# [[ ${RELEASE:=0} -gt 0 ]] && release_flag="--release" || release_flag="" 

action=$1
shift
ACTION=""
MODE=$1

function detect_container_type() {
    case "$MODE" in
        production|dev) 
            echo "=----     Accepting container mode     ----="
            echo $MODE
            return 0  
            echo "=----                                  ----=" 
        ;;
        *) 
            echo "=---- Detecting running container mode ----="
            MODE=$(docker compose -f dockerfiles/docker-compose.yml exec web bash -c "echo \$BUILDMODE") 
            if [[ -z "$MODE" ]]; then
                echo Could not detect mode. ; exit 1; 
            else
                echo Mode: $MODE
            fi 
            return 1; 
        ;;
    esac
} 

case "$action" in
    build)
        detect_container_type && shift 
        ACTION="build"
    ;;
    up)
        detect_container_type && shift 
        ACTION="up"
    ;;
    down)
        detect_container_type && shift 
        ACTION="down"
    ;;
    sh)
        detect_container_type && shift 
        ACTION="exec web bash"
    ;;
    custom)
        detect_container_type && shift
        ACTION=""
    ;; 
    *)
        echo "Not an action: $action" && echo "Actions: build, up, down, sh, custom" 
    ;;
esac




case "$MODE" in
    production) 
       echo "=----                                  ----=" 
       docker compose -f dockerfiles/docker-compose.yml -f dockerfiles/docker-compose.prod.yml $ACTION $@
    ;;
    dev)
       echo "=----                                  ----=" 
       docker compose -f dockerfiles/docker-compose.yml -f dockerfiles/docker-compose.dev.yml $ACTION $@
    ;;
    *)  
        echo "Not a mode: $MODE" && echo "Modes: production, dev" && exit 1
       echo "=----                                  ----=" 
    ;;
esac 

