FROM rust:latest
ENV APP dogrust
WORKDIR /usr/src/$APP
COPY . .
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*
RUN rustup default nightly
RUN cargo install --path .
RUN cargo build -j 16
EXPOSE 8080
CMD ["cargo", "run"]
