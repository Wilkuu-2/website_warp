SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}") 
#CARGO_HOME=$SCRIPT_DIR/.cargo cargo build --release 
ln -s $SCRIPT_DIR/styleMain.css $SCRIPT_DIR/public/styleMain.css

$SCRIPT_DIR/target/release/website_warp 0.0.0.0:8080 no-sec 
