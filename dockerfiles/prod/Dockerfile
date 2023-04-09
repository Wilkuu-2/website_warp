#FROM rust as planner
#WORKDIR opt/app
#COPY . .
#RUN cargo install --config net.git-fetch-with-cli=true cargo-chef
#RUN ./build.sh prep  --recipe-path recipe.json

#FROM rust as cacher
#WORKDIR opt/app
#COPY build.sh .
#COPY --from=planner /opt/app/.cargo .cargo
#COPY --from=planner /opt/app/recipe.json recipe.json
#RUN RELEASE=1 ./build.sh cook --recipe-path recipe.json

FROM rust as builder
WORKDIR opt/app
COPY . . 
# COPY --from=cacher /opt/app/target target
# COPY --from=cacher /opt/app/.cargo .cargo
RUN cargo build --release --bin website_warp 

FROM debian:buster-slim as runtime
WORKDIR opt/app
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
ENV BUILDMODE=production
COPY --from=builder /opt/app/target/release/website_warp /usr/local/bin
CMD ["website_warp", "0.0.0.0:8080", "no-sec"]
EXPOSE 8080 


