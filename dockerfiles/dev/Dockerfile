FROM rust as runner
RUN cargo install cargo-watch
WORKDIR /opt/app
ENV BUILDMODE=dev
CMD ls -la && ./docker_scripts/dev_start.sh


