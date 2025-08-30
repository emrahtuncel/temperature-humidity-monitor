docker run -d \
  --name mosquitto \
  -p 1883:1883 \
  -p 9001:9001 \
  -v ./config:/mosquitto/config/ \
  -v ../data/mosquitto:/mosquitto/data \
  eclipse-mosquitto:latest
