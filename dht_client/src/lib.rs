use chrono::{DateTime, Local};
use prost::Message;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::SystemTime;
use std::{process, thread, time, time::Duration};

extern crate paho_mqtt as mqtt;

#[link(name = "dht")]
extern "C" {
    fn pi_dht_read(sensor: i32, pin: i32, humidity: &mut f32, temperature: &mut f32) -> i32;
}

pub mod dht_sensor {
    include!(concat!(env!("OUT_DIR"), "/dht_client.dht_sensor.rs"));
}

#[derive(Debug, Copy, Clone)]
pub enum Sensor {
    DHT11 = 11,
    DHT22 = 22,
}

impl Sensor {
    pub fn new(sensor_name: &str) -> Option<Sensor> {
        match sensor_name {
            "DHT22" => Option::Some(Sensor::DHT22),
            "DHT11" => Option::Some(Sensor::DHT11),
            _ => Option::None,
        }
    }
}

pub enum DhtResult {
    DhtErrorTimeout = -1,
    DhtErrorChecksum = -2,
    DhtErrorArgument = -3,
    DhtErrorGPIO = -4,
    DhtSuccess = 0,
    DhtInvalid = 1,
}

pub struct Payload {
    time: SystemTime,
    temperature: f32,
    humidity: f32,
}

impl Payload {
    pub fn new(time: SystemTime, temperature: f32, humidity: f32) -> Payload {
        Payload {
            time,
            temperature,
            humidity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    sensor_type: Sensor,
    pin_number: i32,
    device_id: String,
    broker_address: String,
    client_id: String,
    password: String,
    mqtt_topic: String,
    mqtt_qos: i32,
}

impl Config {
    pub fn new(
        sensor_type: Sensor,
        pin_number: i32,
        device_id: String,
        broker_address: String,
        client_id: String,
        password: String,
        mqtt_topic: String,
        mqtt_qos: i32,
    ) -> Config {
        Config {
            sensor_type,
            pin_number,
            device_id,
            broker_address,
            client_id,
            password,
            mqtt_topic,
            mqtt_qos,
        }
    }
}

impl DhtResult {
    fn from_i32(value: i32) -> DhtResult {
        match value {
            -1 => DhtResult::DhtErrorTimeout,
            -2 => DhtResult::DhtErrorChecksum,
            -3 => DhtResult::DhtErrorArgument,
            -4 => DhtResult::DhtErrorGPIO,
            0 => DhtResult::DhtSuccess,
            _ => DhtResult::DhtInvalid,
        }
    }
}

pub fn read_sensor_data(config: Arc<Config>, tx: Sender<Payload>) {
    let sensor_value = config.sensor_type as i32;

    loop {
        let mut humidity: f32 = 0.0;
        let mut temperature: f32 = 0.0;

        let r = unsafe {
            pi_dht_read(
                sensor_value,
                config.pin_number,
                &mut humidity,
                &mut temperature,
            )
        };

        let local: SystemTime = SystemTime::now();

        let _res = match DhtResult::from_i32(r) {
            DhtResult::DhtSuccess => {
                let _res = tx.send(Payload::new(local, temperature, humidity));
            }
            DhtResult::DhtErrorTimeout => println!("Timeout error."),
            DhtResult::DhtErrorChecksum => println!("Checksum error."),
            DhtResult::DhtErrorGPIO => println!("GPIO error."),
            _ => println!("Unexpected result! Time:"),
        };

        let one_secs = time::Duration::from_secs(1);
        thread::sleep(one_secs);
    }
}

pub fn send_sensor_data(config: Arc<Config>, rx: Receiver<Payload>) {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(config.broker_address.clone())
        .client_id(config.client_id.clone())
        .finalize();

    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .user_name(config.client_id.clone())
        .password(config.password.clone())
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    for received in rx {
        let local_time: DateTime<Local> = received.time.into();
        let timestr = local_time.format("%d/%m/%Y %H:%M:%S");
        println!(
            "Time: {} Temperature {}, Humidity {}",
            timestr, received.temperature, received.humidity
        );

        let message_content = create_sensor_data(&received, &config.device_id);

        let msg = mqtt::Message::new(
            config.mqtt_topic.clone(),
            serialize_sensor_data(&message_content),
            config.mqtt_qos,
        );

        let tok = cli.publish(msg);

        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
            break;
        }
    }

    let tok = cli.disconnect(None);
    println!("Disconnect from the broker");
    tok.unwrap();
}

pub fn create_sensor_data(payload: &Payload, device_id: &String) -> dht_sensor::SensorData {
    let mut sensor_data = dht_sensor::SensorData::default();
    sensor_data.device_id = device_id.clone();
    sensor_data.measurement_time = Some(prost_types::Timestamp::from(payload.time));
    sensor_data.temperature = payload.temperature;
    sensor_data.humidity = payload.humidity;
    sensor_data
}

pub fn serialize_sensor_data(sensor_data: &dht_sensor::SensorData) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(sensor_data.encoded_len());

    sensor_data.encode(&mut buf).unwrap();
    buf
}
