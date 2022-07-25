# syntax=docker/dockerfile:1.4

# builder
FROM ekidd/rust-musl-builder as builder
COPY --chown=rust:rust --link . ./
USER rust
RUN \
	--mount=type=cache,target=/usr/local/cargo/registry \
	cargo build --release

# watchdog
FROM ghcr.io/openfaas/of-watchdog:0.9.6 as watchdog

# app
FROM scratch
ENV fprocess="/app" 
COPY --link --from=builder "/home/rust/src/target/x86_64-unknown-linux-musl/release/template-rust" "/app"
COPY --link --from=watchdog "/fwatchdog" "/fwatchdog"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1
CMD ["/fwatchdog"]