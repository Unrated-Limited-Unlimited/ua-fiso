# Use the rust image as base
FROM rust:latest

# Copy the source code into the container
COPY . .

EXPOSE 8001
EXPOSE 27017

CMD ["/target/release/ua-fiso"]
