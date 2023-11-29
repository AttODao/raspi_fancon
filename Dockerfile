FROM rust:alpine AS build
RUN apk update && apk add alpine-sdk
WORKDIR /raspi_fancon
COPY ./ ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=build /raspi_fancon/target/release/raspi_fancon ./
CMD [ "./raspi_fancon" ]