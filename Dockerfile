ARG RUST_VERSION=1.70.0
ARG APP_NAME=server

##################################################################################
# Build stage

FROM rust:${RUST_VERSION}-slim-bullseye AS build
WORKDIR /server

# Copy source code

COPY . .
ADD docker.env .env

# Install the diesel-cli

RUN apt update
RUN apt install -y libsqlite3-0 libsqlite3-dev
RUN cargo install diesel_cli --no-default-features --features sqlite

# Run database migrations

RUN mkdir -p /app/files
RUN diesel migration run

# Build step
RUN cargo build --locked --release && cp ./target/release/$APP_NAME /bin/server

###################################################################################
# Minimal image for final stage

FROM scratch AS final
COPY --from=build /bin/server /bin/
EXPOSE ${PORT}
CMD ["/bin/server"]
