#!/bin/bash

#move to directory of the project
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

#spin up docker and hold script until accepting connections
docker compose up -d
until pg_isready -h localhost -p 5432 -U antonio
do
  echo "Waiting for postgres"
  sleep 2;
done

cargo build
cargo test

#run server in background
cargo run config.yml &
SERVER_PID=$!
sleep 10
diesel migration run

#create the user
curl --location --request POST 'http://localhost:8000/v1/user/create' \
--header 'Content-Type: application/json' \
--data-raw '{
  "name": "maxwell",
  "email": "maxwell@gmail.com",
  "password": "test"
}'
#login getting a fresh token
echo $(curl --location --request GET 'http://localhost:8000/v1/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
  "username": "maxwell",
  "password": "test"
}') > ./fresh_token.json

TOKEN=$(jq '.token' fresh_token.json)
jq '.auth.apikey[0].value = '"$TOKEN"'' scripts/to_do_items.postman_collection.json > test_newman.json

newman run test_newman.json

rm ./test_newman.json
rm ./fresh_token.json

#shut down rust server
kill $SERVER_PID

docker compose down
