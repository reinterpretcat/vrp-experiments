FROM rust:1.46-alpine AS Builder

RUN apk add --no-cache musl-dev

# build app
WORKDIR /src/
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release


FROM alpine:3.12

ENV APP_USER=user
RUN addgroup $APP_USER && adduser -S $APP_USER -G $APP_USER

RUN mkdir /app
COPY --from=Builder /src/target/release/server /app/server
RUN chown -R $APP_USER:$APP_USER /app/server

USER $APP_USER

CMD ["/app/server"]