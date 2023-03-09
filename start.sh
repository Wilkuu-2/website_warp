SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}") 
echo $SCRIPT_DIR
#CARGO_HOME=$SCRIPT_DIR/.cargo cargo build --release 
$SCRIPT_DIR/target/release/website_warp 0.0.0.0:8080 no-sec 
