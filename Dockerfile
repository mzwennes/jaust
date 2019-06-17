ARG BASE_IMAGE=ekidd/rust-musl-builder:nightly-2019-06-08-openssl11

FROM ${BASE_IMAGE} AS builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/jaust /
CMD ["/jaust"]