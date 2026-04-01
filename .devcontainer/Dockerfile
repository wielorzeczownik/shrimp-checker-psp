FROM rust:latest

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        git \
        pkg-config \
        python3 \
        sudo \
        unzip \
        zip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup install nightly \
    && rustup default nightly \
    && rustup component add rustfmt clippy rust-src \
    && cargo install cargo-psp

WORKDIR /workspaces/shrimp-checker-psp

CMD ["/bin/bash"]
