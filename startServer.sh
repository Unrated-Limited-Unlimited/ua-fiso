cargo build --release
docker build -t ua-fiso:latest .
docker stop ua-fiso | docker remove ua-fiso
docker run --network ua-network --name ua-fiso -p 8001:8001 -d ua-fiso:latest
