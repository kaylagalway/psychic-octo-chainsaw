START
docker compose up -d

CONNECT
docker container exec -it <container name> /bin/bash

STOP
docker compose stop



ROUTES
GET index

POST /login

GET /user
POST /user/abilities

GET /feed
GET /workout/:ID


