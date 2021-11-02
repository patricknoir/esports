#!/bin/zsh
docker-compose down &&
sleep 15s &&
docker-compose up &
sleep 30s &&
cd account-service &&
diesel setup &&
cd .. &&