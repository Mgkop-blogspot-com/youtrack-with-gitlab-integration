# Phase 1
FROM drenozua/rust_opencv:latest AS builder

ARG PROFILE=release

## Project first build
RUN USER=root cargo new --bin image-loading-api && \
    mkdir -p image-loading-api/app/src && mv image-loading-api/src/main.rs image-loading-api/app/src/main.rs
RUN USER=root cargo new --lib image-loading-api/sdk/api-files
RUN USER=root cargo new --lib image-loading-api/sdk/sample-convertors
RUN USER=root cargo new --lib image-loading-api/sdk/sample-image-tools
RUN USER=root cargo new --lib image-loading-api/sdk/sample-io
RUN USER=root cargo new --lib image-loading-api/sdk/sample-storage

WORKDIR ./image-loading-api

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
#COPY ./app/Cargo.toml ./app/Cargo.toml
COPY ./sdk/api-files/Cargo.toml ./sdk/api-files/Cargo.toml
COPY ./sdk/sample-convertors/Cargo.toml ./sdk/sample-convertors/Cargo.toml
COPY ./sdk/sample-image-tools/Cargo.toml ./sdk/sample-image-tools/Cargo.toml
COPY ./sdk/sample-io/Cargo.toml ./sdk/sample-io/Cargo.toml
COPY ./sdk/sample-storage/Cargo.toml ./sdk/sample-storage/Cargo.toml

RUN cargo build --$PROFILE && \
    find app -name "*.rs" -type f -print0 | xargs -0 /bin/rm -f && \
    find sdk -name "*.rs" -type f -print0 | xargs -0 /bin/rm -f && \
    find ./target/release/deps -name "web_app*" -type f -print0 | xargs -0 /bin/rm -f && \
    find ./target/release/deps -name "*api_files*" -type f -print0 | xargs -0 /bin/rm -f && \
    find ./target/release/deps -name "*sample_image_tools*" -type f -print0 | xargs -0 /bin/rm -f && \
    find ./target/release/deps -name "*sample_storage*" -type f -print0 | xargs -0 /bin/rm -f && \
    find ./target/release/deps -name "*sample_convertors*" -type f -print0 | xargs -0 /bin/rm -f && \
    find ./target/release/deps -name "*sample_io*" -type f -print0 | xargs -0 /bin/rm -f
##

ADD . ./

# second build
RUN cargo build --$PROFILE

# Phase 2
FROM drenozua/rust_opencv:latest

ARG PROFILE=release
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata net-tools \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /go/image-loading-api/target/release/web-app ${APP}/web-app
COPY --from=builder /go/image-loading-api/settings.yaml ${APP}/settings.yaml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./web-app"]