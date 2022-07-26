
#!/bin/sh

# Steps to setup ec2 with docker-compose

sudo yum update 
sudo yum install docker git python3-pip

sudo usermod -a -G docker ec2-user 
id ec2-user 
newgrp docker  

sudo pip3 install docker-compose 

sudo systemctl enable docker.service 
sudo systemctl start docker.service 

set -e 
sudo systemctl status docker.service
docker-compose up -d
