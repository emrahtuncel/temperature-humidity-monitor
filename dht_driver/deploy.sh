PI_IP=192.168.2.252 # Be sure to change this!

# upload binary
sshpass -p 'raspberry' scp -r ./sensor pi@$PI_IP:/home/pi

# execute binary
sshpass -p 'raspberry' ssh pi@$PI_IP
