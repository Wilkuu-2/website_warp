if [ ${WATCH:=0} -gt 0 ] 
then
    echo "Watching for changes..." 
    cargo watch -x 'run -- 0.0.0.0:8080 no-sec'
else
    echo "Straight to build..."
    cargo run -- 0.0.0.0:8080 no-sec
fi 
