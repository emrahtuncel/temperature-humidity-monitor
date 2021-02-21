FROM tttapa/rpi-cross-toolchain:armv6-rpi-linux-gnueabihf
LABEL maintainer="Emrah Tuncel <etuncel88@gmail.com>"

ENV PROTOBUF_VERSION=3.15.1
ENV GO_VERSION=1.16

USER root
RUN yum install sshpass cmake -y
RUN ln -s /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/armv6-rpi-linux-gnueabihf-gcc /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc && \
      ln -s /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/armv6-rpi-linux-gnueabihf-g++ /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-g++ && \
      chown develop:develop /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc && \
      chown develop:develop /home/develop/x-tools/armv6-rpi-linux-gnueabihf/bin/arm-linux-gnueabihf-g++
RUN cd /home/develop/src && wget https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOBUF_VERSION}/protobuf-all-${PROTOBUF_VERSION}.tar.gz && \
        tar -xzf protobuf-all-${PROTOBUF_VERSION}.tar.gz && cd protobuf-${PROTOBUF_VERSION} && \
        ./configure --prefix=/usr && make && make install && ldconfig && cd .. && rm -rf protobuf-${PROTOBUF_VERSION}
RUN cd /home/develop/src && wget https://golang.org/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
        tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz

USER develop
ENV GOPATH=/home/develop/go
ENV PATH=${GOPATH}/bin:/usr/local/go/bin:${PATH}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source /home/develop/.cargo/env && rustup target add arm-unknown-linux-gnueabihf
