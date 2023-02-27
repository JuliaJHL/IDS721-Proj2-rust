FROM rust:latest as builder
ENV APP dog-recommendations
WORKDIR /usr/src/$APP
COPY . .
COPY data data
COPY templates templates 
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["dog-recommendations"]