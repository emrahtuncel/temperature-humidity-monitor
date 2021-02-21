use dht_client::{Sensor, read_sensor_data, send_sensor_data, Config};
use std::{sync::mpsc, sync::Arc, thread};
use serde_yaml::Value;

pub fn main() {

    let producer_config = Arc::new(parse_config());
    let consumer_config = producer_config.clone();

    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || {
        read_sensor_data(producer_config, tx);
    });

    let consumer = thread::spawn(move || {
        send_sensor_data(consumer_config, rx);
    });

    let _res = producer.join();
    let _res = consumer.join();

}

pub fn parse_config() -> Config {

    let config_file = std::fs::File::open("conf/config.yaml").unwrap();
    let config_values : Value = serde_yaml::from_reader(config_file).unwrap();
    let config = Config::new(
        Sensor::new(config_values["sensor-type"].as_str().unwrap()).unwrap(),
        config_values["pin-number"].as_i64().unwrap() as i32,
        String::from(config_values["device-id"].as_str().unwrap()),
        String::from(config_values["broker-address"].as_str().unwrap()),
        String::from(config_values["client-id"].as_str().unwrap()),
        String::from(config_values["password"].as_str().unwrap()),
        String::from(config_values["mqtt-topic"].as_str().unwrap()),
        config_values["mqtt-qos"].as_i64().unwrap() as i32,
    );
    println!("{:?}", config);
    config
}
