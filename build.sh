SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}") 
echo $SCRIPT_DIR
CARGO_HOME=$SCRIPT_DIR/.cargo cargo build --release 
#$SCRIPT_DIR/target/release/website_warp "$IP:80" lets-encrypt jakub@wilkuu.xyz wilkuu.xyz
