FROM rust as runner
RUN cargo install cargo-watch
WORKDIR /opt/app
COPY ./.cargo .
COPY ./docker_scripts/dev_start.sh .
CMD ./start.sh


