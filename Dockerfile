ARG RUST_VERSION=1.70.0
ARG APP_NAME=point_cloud_server

FROM rust:${RUST_VERSION}-slim-bullseye AS final

WORKDIR /${APP_NAME}

# Copy source code

COPY . .
ADD docker.env .env

# Install the diesel-cli

RUN apt update
RUN apt install -y libsqlite3-0 libsqlite3-dev
RUN cargo install diesel_cli --no-default-features --features sqlite

# Run database migrations

RUN mkdir -p files
RUN diesel migration run

# Build step
RUN cargo build --locked --release

EXPOSE ${PORT}
ENTRYPOINT [ "./target/release/point_cloud_server" ]