FROM debian:bookworm as runtime
LABEL org.opencontainers.image.source=https://github.com/NINNiT/rust-template
LABEL org.opencontainers.image.description="A template for rust projects"

COPY ./target/release/rust-template /usr/bin/rust-template
RUN chmod +x /usr/bin/rust-template && \
  mkdir -p /etc/rust-template
COPY config.toml /etc/rust-template/config.toml

ENTRYPOINT ["/usr/bin/rust-template"]
CMD ["--config", "/etc/rust-template/config.toml", "daemon"]
