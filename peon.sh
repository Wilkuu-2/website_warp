#!/bin/bash

set -e

# [[ ${RELEASE:=0} -gt 0 ]] && release_flag="--release" || release_flag="" 

action=$1
shift
MODE=""
RUST_FLAGS=""
RUST_LOG_DEFAULT="web::main=error,web::projects::track=info"
if [ -z $RUST_LOG ] 
then
    RUST_LOG="$RUST_LOG_DEFAULT"
else 
    RUST_LOG="$RUST_LOG,$RUST_LOG_DEFAULT"
fi

function mode_parse() {
    case "$1" in
        production) 
            echo "Mode: Production"
            MODE="prod"
            RUST_FLAGS="$RUST_FLAGS --release"
            WATCH=0
            ;;
        dev)
            echo "Mode: Development"
            MODE="dev"
            RUST_FLAGS="$RUST_FLAGS --features=debug"
            export RUST_BACKTRACE=1
            ;;
        *)  
            echo "Not a mode: $MODE" && echo "Modes: production, dev" && echo "Assuming 'dev'"  
            mode_parse "dev" 
            ;;
    esac 
}

mode_parse $1

function peon_down() {
    kill $(cat web.pid) || (echo "App is not running" ;  exit) 
    rm web.pid
}

function peon_up() {
        if [[ ! -z $(cat web.pid 2>> /dev/null) ]] 
        then
            echo "App already running"
            exit 1
        fi

        (peon_run >>log/website.log 2>>log/website.err; rm web.pid) &
        echo $! > web.pid 
}

function peon_run(){
        if [[ ! -z $(cat web.pid 2>> /dev/null) ]] 
        then
            peon_down 
        fi
        
        if [ ${WATCH:=0} -gt 0 ] 
        then
            echo "Watching for changes..."
            cargo watch -x "run $RUST_FLAGS"
        else 
            echo "Building and running once..."
            cargo run $RUST_FLAGS
        fi
    
}


function peon_restart(){
        # restart when already running 
        if [[ ! -z $(cat web.pid 2>> /dev/null) ]] 
        then
            peon_down 
        fi
        
        peon_up
}

l_actions=("build" "up" "run" "down" "restart")

case "$action" in
    ${l_actions[0]})
        echo "== Building == "
        cargo build $RUST_FLAGS
    ;;
    ${l_actions[1]})
        echo "== Starting up as daemon == "
        peon_up
    ;;
    
    ${l_actions[2]})
        echo "== Starting up == "
        peon_run
    ;;
    
    ${l_actions[3]})
        echo "== Shutting down == "
        peon_down
        
    ;;
    ${l_actions[4]})
        echo "== Restarting == "
        peon_restart
    ;;
    *)
        echo "Not an action: $action" 
        echo "Actions:"

        for act in ${l_actions[@]}; do
            echo "    - $act"
        done 

        exit 1 
    ;;
esac





