PI_IP=192.168.2.252 # Be sure to change this!
TARGET=arm-unknown-linux-gnueabihf # Pi 0/1

# upload binary
sshpass -p 'raspberry' scp -r ./target/$TARGET/debug/dht_client pi@$PI_IP:/home/pi
sshpass -p 'raspberry' scp -r ./conf pi@$PI_IP:/home/pi

# execute binary
sshpass -p 'raspberry' ssh pi@$PI_IP
