# Temperature and Humidity Monitor

This project is a simple solution for monitoring temperature and humidity data using a Raspberry Pi. It consists of a sensor client, an IoT server, and a set of Docker containers for the necessary services.

## Submodules

The project is divided into the following submodules:

*   **dht_driver:** A C program that reads temperature and humidity data from a DHT sensor connected to a Raspberry Pi.
*   **dht_client:** A Rust program that reads sensor data using the `dht_driver` and publishes it to an MQTT broker.
*   **iot_server:** A Go program that subscribes to the MQTT topic, receives the sensor data, and stores it in an InfluxDB database.
*   **docker-mosquitto:** A Docker container for the Eclipse Mosquitto MQTT broker.
*   **docker-influxdb-grafana:** A Docker container that runs InfluxDB for time-series data storage and Grafana for data visualization.
*   **docker-raspberry-build-env:** A Docker container that provides a cross-compilation environment for building the `dht_client` and `iot_server` for the Raspberry Pi.

## Development Environment Setup

To set up the development environment, you need to build and run the Docker containers.

### 1. Build the Build Environment Container

This container is used to build the `dht_client` and `iot_server`.

```bash
cd docker-raspberry-build-env
./build.sh
```

### 2. Build the `dht_client` and `iot_server`

Once you are inside the build environment container, you can build the applications.

#### Build `dht_client`

```bash
cd /home/develop/temperature-humidity-monitor/dht_client
cargo build --target=arm-unknown-linux-gnueabihf --release
```

#### Build `iot_server`

```bash
cd /home/develop/temperature-humidity-monitor/iot_server
GOOS=linux GOARCH=arm GOARM=6 go build -o iot_server
```

### 3. Run the MQTT Broker

This container runs the Eclipse Mosquitto MQTT broker.

```bash
cd docker-mosquitto
./build.sh
```

### 4. Run InfluxDB and Grafana

This container runs InfluxDB and Grafana.

```bash
cd docker-influxdb-grafana
./build.sh
```

## Deployment and Running the Project

To deploy and run the project, follow these steps:

1.  **Deploy the `dht_client` to the Raspberry Pi:**
    *   Copy the compiled `dht_client` binary from the build environment to your Raspberry Pi.
    *   Copy the `dht_client/conf/config.yaml` file to the Raspberry Pi and update the `broker-address` to the IP address of the machine running the MQTT broker.
    *   Run the `dht_client` on the Raspberry Pi.

2.  **Deploy the `iot_server`:**
    *   Copy the compiled `iot_server` binary to the machine where you will run the server.
    *   Copy the `iot_server/conf/config.yaml` file and update the `brokeraddress` and `influxdbaddress` to the correct IP addresses.
    *   Run the `iot_server`.

3.  **Run the Docker Containers:**
    *   Run the `docker-mosquitto` and `docker-influxdb-grafana` containers as described in the "Development Environment Setup" section.

4.  **Configure Grafana:**
    *   Open Grafana in your browser (usually at `http://<machine-ip>:3000`).
    *   Log in with the default credentials (admin/admin).
    *   Add a new data source for InfluxDB, using the IP address of the InfluxDB container.
    *   Create a new dashboard and add panels to visualize the temperature and humidity data.
