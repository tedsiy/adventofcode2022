FROM rust:1.65.0-alpine3.16 as builder

LABEL maintainer="Teddy Siy <teddy.siy@aptiv.com>"

WORKDIR /root/day01

# Copy source directory files to container
COPY Cargo.toml /root/day01/
COPY sample.txt /root/day01/
COPY src/ /root/day01/src/

# RUSTFLAGS='-C target-feature=+crt-static' 
RUN cargo build --release

############################
# BUILD 2 make a small image
############################
# https://github.com/GoogleContainerTools/distroless/blob/main/base/README.md
# use static for statically compiled languages
FROM gcr.io/distroless/static:nonroot

LABEL maintainer="Teddy Siy <teddy.siy@aptiv.com>"

# Import from builder.
COPY --from=builder /root/day01/target/release/day01 /day01
COPY --from=builder /root/day01/sample.txt /sample.txt

# Run the binary.
CMD ["/day01", "/sample.txt"]