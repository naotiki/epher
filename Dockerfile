FROM ghcr.io/naotiki/nitkc-deb11:latest

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env" && rustup override set stable && rustup update stable
ENV PATH=/root/.cargo/bin:$PATH

CMD cargo build -r