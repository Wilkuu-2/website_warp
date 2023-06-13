if [ ${WATCH:=0} -gt 0 ] 
then
    echo "Watching for changes..." 
    RUST_LOG="web::main=error,web::projects::track=info" cargo watch -x 'run --features=debug'
else
    echo "Straight to build..."
    RUST_LOG="web::main=error,web::projects::track=info"
    cargo run --features=debug 
fi 
