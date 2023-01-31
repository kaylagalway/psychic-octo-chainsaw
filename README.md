CREATE
docker build -t <image name> .

START
docker run <image name>

CONNECT
docker container exec -it <container name> /bin/bash

STOP
docker stop <container id>