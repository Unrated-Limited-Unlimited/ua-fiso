# Use the rust image as base
FROM rust:latest

WORKDIR ua-fiso

# Copy the source code into the container
COPY /target/release/ .

EXPOSE 8001
EXPOSE 27017

CMD ["./ua-fiso"]

