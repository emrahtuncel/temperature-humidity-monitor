#docker build . -t emrah/raspberry-build:1.0

docker run -it --rm \
  --name raspberry-build \
  -v ..:/home/develop/temperature-humidity-monitor/ \
  emrah/raspberry-build:1.0 \
  bash
