# SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
#
# SPDX-License-Identifier: AGPL-3.0-or-later
FROM rust:slim as rust
WORKDIR /src
RUN apt-get update && apt-get install -y git pkg-config libssl-dev build-essential
RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release
COPY . /src
RUN cargo build --release

FROM debian:bookworm as mcaptcha-cli
LABEL org.opencontainers.image.source https://github.com/mcaptcha/cli
RUN useradd -ms /bin/bash -u 1001 mcaptcha-cli
WORKDIR /home/mcaptcha-cli
COPY --from=rust /src/target/release/mcaptcha-cli /usr/local/bin/
USER mcaptcha-cli
CMD [ "/usr/local/bin/mcaptcha-cli" ]
