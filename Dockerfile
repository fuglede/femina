FROM ekidd/rust-musl-builder:nightly-2019-04-25 as builder
WORKDIR /build
COPY . .
RUN sudo chown -R rust:rust /build
RUN cargo build --release

FROM alpine 
EXPOSE 8000
WORKDIR /root
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/femina .
CMD ["./femina"]
