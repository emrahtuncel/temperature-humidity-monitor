#docker build . -t emrah/influxdb-grafana:1.0

docker run -d \
  --name influxdb-grafana \
  -p 3003:3003 \
  -p 8086:8086 \
  -v /home/emrah/db/influxdb:/var/lib/influxdb \
  -v /home/emrah/db/grafana:/var/lib/grafana \
  emrah/influxdb-grafana:1.0
