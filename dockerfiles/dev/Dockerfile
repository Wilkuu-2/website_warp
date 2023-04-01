FROM rust as runner
RUN cargo install cargo-watch
WORKDIR /opt/app
COPY ./start.sh .
CMD ./start.sh


