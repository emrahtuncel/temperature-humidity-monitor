package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	influxdb2 "github.com/influxdata/influxdb-client-go/v2"
	proto "google.golang.org/protobuf/proto"
	yaml "gopkg.in/yaml.v2"
)

// Config ...
type Config struct {
	BrokerAddress   string
	ClientID        string
	Password        string
	MqttTopic       string
	MqttQos         int
	InfluxdbAddress string
	InfluxdbToken   string
	DbOrganization  string
	DbBucket        string
}

var channel chan *SensorData
var config Config

var messagePubHandler mqtt.MessageHandler = func(client mqtt.Client, msg mqtt.Message) {
	sensorData := &SensorData{}
	proto.Unmarshal(msg.Payload(), sensorData)

	fmt.Printf("Received message from topic: %s\n", msg.Topic())

	channel <- sensorData
}

var connectHandler mqtt.OnConnectHandler = func(client mqtt.Client) {
	fmt.Println("Connected")
}

var connectLostHandler mqtt.ConnectionLostHandler = func(client mqtt.Client, err error) {
	fmt.Printf("Connect lost: %v\n", err)
}

func parseConfig() {

	filename, _ := filepath.Abs("conf/config.yaml")
	yamlFile, err := os.ReadFile(filename)

	if err != nil {
		panic(err)
	}

	err = yaml.Unmarshal(yamlFile, &config)
	if err != nil {
		panic(err)
	}

	res, _ := json.Marshal(config)
	fmt.Println(string(res))
}

func initializeBrokerConnection() mqtt.Client {

	opts := mqtt.NewClientOptions()
	opts.AddBroker(config.BrokerAddress)
	opts.SetClientID(config.ClientID)
	opts.SetUsername(config.ClientID)
	opts.SetPassword(config.Password)
	opts.SetDefaultPublishHandler(messagePubHandler)
	opts.OnConnect = connectHandler
	opts.OnConnectionLost = connectLostHandler
	client := mqtt.NewClient(opts)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}
	sub(client)

	return client
}

func closeBrokerConnection(client mqtt.Client) {
	client.Disconnect(250)
}

func initializeDbConnection() influxdb2.Client {
	dbClient := influxdb2.NewClient(config.InfluxdbAddress, config.InfluxdbToken)
	return dbClient
}

func closeDbConnection(dbClient influxdb2.Client) {
	dbClient.Close()
}

func main() {

	parseConfig()

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)

	dbClient := initializeDbConnection()
	client := initializeBrokerConnection()

	channel = make(chan *SensorData)
	go dataConsumer(dbClient)

	<-c
	closeBrokerConnection(client)
	closeDbConnection(dbClient)
}

func sub(client mqtt.Client) {
	token := client.Subscribe(config.MqttTopic, byte(config.MqttQos), messagePubHandler)
	token.Wait()
	fmt.Printf("Subscribed to topic: %s\n", config.MqttTopic)
}

func dataConsumer(dbClient influxdb2.Client) {

	for sensorData := range channel {

		dateTime := sensorData.GetMeasurementTime().AsTime()
		dateTimeString := dateTime.Local().Format("02/01/2006 15:04:05")

		fmt.Printf("Device: %s, Time: %s, Temperature: %.1f, Humidity: %.1f\n",
			sensorData.GetDeviceId(), dateTimeString, sensorData.GetTemperature(), sensorData.GetHumidity())

		writeAPI := dbClient.WriteAPIBlocking(config.DbOrganization, config.DbBucket)
		// create point using full params constructor
		p := influxdb2.NewPoint("humidity-temperature",
			map[string]string{"device": sensorData.GetDeviceId()},
			map[string]interface{}{"humidity": sensorData.GetHumidity(), "temperature": sensorData.GetTemperature()},
			dateTime)
		// write point immediately
		err := writeAPI.WritePoint(context.Background(), p)
		if err != nil {
			fmt.Println(err)
		}
	}
}
