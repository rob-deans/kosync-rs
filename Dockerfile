FROM rust:1.94 AS backend

# Create app directory
WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

# Copy full backend source
# RUN rm -rf src
COPY src/ ./src/
COPY ./toasty ./toasty/

# Final build with real source
RUN cargo build --release

# Stage 3: Final image using distroless
FROM gcr.io/distroless/cc-debian12:nonroot AS final

# Set workdir
WORKDIR /app

# Copy Rust binary
COPY --from=backend /app/target/release/kosync-rs /app/kosync-rs
COPY --from=backend /app/toasty /app/toasty

# Set the entrypoint to run the binary
ENTRYPOINT ["/app/kosync-rs"]

