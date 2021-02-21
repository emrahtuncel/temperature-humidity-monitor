#docker build . -t emrah/raspberry-build:1.0

docker run -it --rm \
  --name raspberry-build \
  -v /home/emrah/rust_workspace/temperature-humidity-monitor:/home/develop/temperature-humidity-monitor/ \
  emrah/raspberry-build:1.0 \
  bash
