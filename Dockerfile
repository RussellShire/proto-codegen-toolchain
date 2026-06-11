FROM rust:1.88-bookworm

ARG NODE_VERSION=22

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        gnupg \
        protobuf-compiler \
        libprotobuf-dev \
    && curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && cargo install just \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

