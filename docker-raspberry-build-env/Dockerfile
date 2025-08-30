FROM tttapa/rpi-cross-toolchain:armv6-rpi-linux-gnueabihf
LABEL maintainer="Emrah Tuncel <etuncel88@gmail.com>"

ENV GO_VERSION=1.25.0
ENV PROTOC_VERSION=32.0

SHELL ["/bin/bash", "-c"]
USER root
RUN apt-get update
RUN apt-get install sshpass cmake unzip curl -y
RUN ln -s /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/armv6-rpi-linux-gnueabihf-gcc /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc && \
      ln -s /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/armv6-rpi-linux-gnueabihf-g++ /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-g++ && \
      chown develop:develop /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc && \
      chown develop:develop /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-g++
RUN cd /home/develop/src && wget https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
      tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz
USER develop
ENV GOPATH=/home/develop/go
ENV PATH=${GOPATH}/bin:/usr/local/go/bin:${PATH}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source /home/develop/.cargo/env && rustup target add arm-unknown-linux-gnueabihf
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip && \
      unzip protoc-${PROTOC_VERSION}-linux-x86_64.zip -d /home/develop/.local
ENV PATH=/home/develop/.local/bin:${PATH}

