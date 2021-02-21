
#include <stdio.h>
#include <time.h>

#include "pi_dht_read.h"

int main(int argc, char *argv[]) {

    int pinNo = 4;
    int sensor = DHT22;

    float humidity = 0;
    float temperature = 0;

    time_t timer;
    char buffer[26];
    struct tm* tm_info;

    while(1) {

        timer = time(NULL);
        tm_info = localtime(&timer);

        strftime(buffer, 26, "%d-%m-%Y %H:%M:%S", tm_info);

        int result = pi_dht_read(sensor, pinNo, &humidity, &temperature);

        if(result == DHT_SUCCESS) {
            printf("Time: %s Temperature: %.1f, Humidity: %.1f\n",buffer, temperature, humidity);
        }
        else if(result == DHT_ERROR_CHECKSUM) {
            printf("Error checksum. Time: %s\n", buffer);
        }
        else if(result == DHT_ERROR_GPIO) {
            printf("Error GPIO. Time: %s\n", buffer);
        }
        else if(result == DHT_ERROR_TIMEOUT) {
            printf("Error timeout. Time: %s\n", buffer);
        }
    }

}