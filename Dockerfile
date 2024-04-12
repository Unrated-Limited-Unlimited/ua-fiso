# Use the rust image as base
FROM rust:latest

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

EXPOSE 8001

CMD ["/target/release/ua-fiso"]
