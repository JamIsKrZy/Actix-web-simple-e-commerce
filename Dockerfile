FROM rust:1.89-bookworm AS builder 
ENV SQLX_OFFLINE=true

WORKDIR /app/

# crate app
RUN cargo new ./crates/app --bin
RUN mkdir -p ./crates/app/src/bin \
    && echo 'fn main() { println!("Hello from main.rs bin!"); }' > ./crates/app/src/bin/main.rs 
COPY ./crates/app/Cargo.toml ./crates/app/Cargo.toml 


# Lib crates for db-core
RUN cargo new ./crates/libs/db-core --lib
COPY ./crates/libs/db-core/Cargo.toml ./crates/libs/db-core/Cargo.toml

# Lib crate for extensions
RUN cargo new ./crates/libs/extension --lib
COPY ./crates/libs/extension/Cargo.toml ./crates/libs/extension/Cargo.toml

# lib crate for lib-core 
RUN cargo new ./crates/libs/lib-core  --lib
COPY ./crates/libs/lib-core/Cargo.toml ./crates/libs/lib-core/Cargo.toml

# Lib crate for suppoer-core 
RUN cargo new ./crates/libs/support-core --lib
COPY ./crates/libs/support-core/Cargo.toml ./crates/libs/support-core/Cargo.toml


# BUILDS THE DEPENDENCIES
COPY ./Cargo.toml ./Cargo.lock .
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release --bin main



# ---------------------------
#   COPY THE SOURCE FILES 
# ---------------------------
COPY ./crates/app ./crates/app
RUN rm ./crates/app/src/bin/shuttle.rs

COPY ./crates/libs/db-core ./crates/libs/db-core
COPY ./crates/libs/lib-core ./crates/libs/lib-core
COPY ./crates/libs/extension ./crates/libs/extension
COPY ./crates/libs/support-core ./crates/libs/support-core
COPY .sqlx/ .sqlx/
COPY ./migrations/ ./migrations/
COPY ./static/ ./static/

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  # update timestamps to force a new build
  touch ./crates/app/src/bin/main.rs ./crates/libs/db-core/src/lib.rs \
    ./crates/libs/extension/src/lib.rs ./crates/libs/lib-core/src/lib.rs \
    ./crates/libs/support-core/src/lib.rs
  cargo build --release --bin main
EOF

CMD ["/app/target/release/my-app"]

# ----------------------------
#          NEXT STAGE 
# ----------------------------
FROM debian:bookworm-slim 
ARG APP=/user/local/bin

RUN apt-get update \
    && apt install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/

ENV TZ=Etc/UTC \
    APP_USER=appuser
    

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /app/target/release/main ${APP}/app

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./app"]
EXPOSE 8000





