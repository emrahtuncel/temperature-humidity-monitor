docker run -d \
  --name mosquitto \
  -p 1883:1883 \
  -p 9001:9001 \
  -v /home/emrah/db/mosquitto/config:/mosquitto/config/ \
  -v /home/emrah/db/mosquitto/data:/mosquitto/data \
  eclipse-mosquitto:latest
