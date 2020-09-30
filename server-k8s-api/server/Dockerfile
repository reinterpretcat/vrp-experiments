FROM rust:1.46-alpine AS Builder

RUN apk add --no-cache musl-dev

# build app
WORKDIR /src/
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release


FROM alpine:3.12

RUN apk --no-cache add ca-certificates

ENV APP_USER=user
ENV UID=1000
ENV GID=1000
RUN addgroup --gid=${GID} $APP_USER && adduser -S $APP_USER --uid=${UID} -G $APP_USER

RUN mkdir /app
COPY --from=Builder /src/target/release/server /usr/local/bin/server

RUN chown -R $APP_USER:$APP_USER /usr/local/bin/server

USER ${UID}:${GID}

CMD ["/usr/local/bin/server"]