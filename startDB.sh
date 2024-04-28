docker stop ua-fiso-db | docker remove ua-fiso-db
docker run --restart=always --network ua-network --name ua-fiso-db -p 27017:27017 -d mongo --bind_ip_all
