FROM rust:1.78.0-slim-buster as builder

WORKDIR /build

COPY .. /build

RUN apt update -y && \
    apt install -y wget xz-utils elfutils && \
    wget https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz && \
    unxz upx-4.0.2-amd64_linux.tar.xz && tar xvf upx-4.0.2-amd64_linux.tar && \
        cp upx-4.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx && \
    cargo test && \
    cargo build --release && \
    cp target/release/kubectl-search . && \
    eu-elfcompress kubectl-search && \
    strip kubectl-search && \
    upx -9 --lzma kubectl-search

FROM scratch

COPY --from=builder /build/kubectl-search /kubectl-search