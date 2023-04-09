if [ ${WATCH:=0} -gt 0 ] 
then
    echo "Watching for changes..." 
    cargo watch --features=debug -x 'run -- 0.0.0.0:8080 no-sec'
else
    echo "Straight to build..."
    cargo --features=debug run -- 0.0.0.0:8080 no-sec
fi 
