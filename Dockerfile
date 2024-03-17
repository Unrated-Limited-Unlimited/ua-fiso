# Use the rust image as base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/ua-fiso

# Copy the Cargo.toml and Cargo.lock to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies without the entire source code

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/ua-fiso"]