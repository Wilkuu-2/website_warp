FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN ./build.sh prep  --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY build.sh .
COPY --from=planner /app/recipe.json recipe.json
RUN RELEASE=1 ./build.sh cook --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . . 
COPY --from=cacher /app/target target
RUN RELEASE=1 ./build.sh build --bin website_warp

FROM rust as runtime
WORKDIR opt/app 
COPY --from=builder /app/target/release/website_warp /usr/local/bin
RUN ls
CMD ["website_warp", "0.0.0.0:8080", "no-sec"]
EXPOSE 8080 


