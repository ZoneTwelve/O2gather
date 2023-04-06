FROM rust:1.68 as builder

WORKDIR /usr/src/app
COPY . .
#RUN # --mount=type=cache,target=/usr/local/cargo/registry \
    # --mount=type=cache,target=/usr/src/app/target \
    #cargo build --release
RUN cargo build --release
RUN ls -alh

FROM ubuntu:20.04
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/backend ./backend
# copy the configration
RUN ls -alh
CMD ["./backend"]
