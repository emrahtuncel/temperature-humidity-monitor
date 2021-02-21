#docker build . -t emrah/raspberry-build:1.0

docker run -it --rm \
  --name raspberry-build \
  -v /home/emrah/rust_workspace/:/home/develop/rust_workspace/ \
  emrah/raspberry-build:1.0 \
  bash
