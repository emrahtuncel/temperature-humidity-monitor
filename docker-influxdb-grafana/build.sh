#docker build . -t emrah/influxdb-grafana:1.0

docker run -d \
  --name influxdb-grafana \
  -p 3003:3003 \
  -p 8086:8086 \
  -v ../data/influxdb:/var/lib/influxdb \
  -v ../data/grafana:/var/lib/grafana \
  emrah/influxdb-grafana:1.0
