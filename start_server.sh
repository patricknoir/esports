#!/bin/zsh
docker-compose down &&
sleep 10s &&
docker-compose up &
sleep 20s &&
cd account-service &&
diesel setup &&
cd .. &&