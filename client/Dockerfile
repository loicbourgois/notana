FROM rust
RUN rustup override set nightly
COPY backend/Cargo.toml /root/github.com/loicbourgois/taskini/backend/Cargo.toml
COPY backend/src/main.rs /root/github.com/loicbourgois/taskini/backend/src/main.rs
RUN --mount=type=cache,target=/root/.cargo cargo fetch --manifest-path /root/github.com/loicbourgois/taskini/backend/Cargo.toml
COPY backend /root/github.com/loicbourgois/taskini/backend
# RUN --mount=type=cache,target=/root/.cargo cargo install --path /root/github.com/loicbourgois/taskini/backend
