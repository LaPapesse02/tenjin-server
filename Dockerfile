FROM rust:1.99.1-alpine AS build

ARG APP_NAME=tenjin-server-rs

WORKDIR /app

COPY . .
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM alpine AS final
COPY --from=build /bin/server /bin/

CMD ["/bin/server"]