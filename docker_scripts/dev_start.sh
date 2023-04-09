if [ ${WATCH:=0} -gt 0 ] 
then
    echo "Watching for changes..." 
    cargo watch --features=debug -x 'run'
else
    echo "Straight to build..."
    cargo --features=debug run
fi 
