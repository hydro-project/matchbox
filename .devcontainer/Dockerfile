FROM debian:bullseye
ENV LC_ALL C.UTF-8
RUN apt-get update && apt-get install -y apt-utils gcc curl less nano mc whiptail locales man-db wget procps sudo apt-file pkg-config

# User
RUN useradd --create-home --uid 1000 --shell /bin/bash user
RUN echo 'user ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/a
USER user
WORKDIR /home/user

# Rust
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain none
RUN . "$HOME/.cargo/env"; rustup toolchain install nightly --allow-downgrade
