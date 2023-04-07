FROM rust:1.68 as builder

WORKDIR /usr/src/app
COPY . .
#RUN # --mount=type=cache,target=/usr/local/cargo/registry \
    # --mount=type=cache,target=/usr/src/app/target \
    #cargo build --release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

FROM rust:1.68
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/backend ./backend
# copy the configration
## Do something
CMD ["./backend"]
