# Installation

## Minikube

```bash
$minikube start
...
$minikube addons enable ingress
...
```

## Postgres Setup

```bash
$cd k8s
k8s$kubectl apply -f postgres.yml
...
k8s$kubectl port-forward service/postgres-service 30001:5432
```

on a new shell:
```bash
$cd account-service
account-service$diesel setup
...
```

on the previous shell terminate the port forwarding:
```bash
...
k8s$kubectl port-forward service/postgres-service 30001:5432
^C
k8s$kubectl apply -f esports.yaml
...
k8s$kubectl apply -f core.yaml
```

Apply tunneling from minikube:
```bash
$minikube tunnel
...
password: xxxx
ingress running on port 80
```

## Usage

### Create User

#### Request

Method: POST

URL: http://localhost/users

Headers:
  
    Content-Type: application/json

Payload:
```json
{
  "profilePicture":"/avatar/Avatar-10.png",
  "email":"john.smith@gmail.com",
  "username":"john",
  "phone":"+35059118440",
  "password":"password123"
}
```

#### Response:

Header:

    Content-Type: application/json
    Authorization: Bearer <JWT TOKEN>

Payload:
```json
{
"id": "f36a81af-11d5-4c55-acd7-edb00f69ba0a",
"profilePicture": "/avatar/Avatar-10.png",
"username": "john",
"email": "john.smith@gmail.com",
"phone": "+35059118440",
"role": "Player",
"isActive": true,
"createdDate": "2022-04-07T15:20:18.925852",
"updatedDate": "2022-04-07T15:20:18.925852"
}
```

### Login

#### Request

Method: POST

URL: http://localhost/login

Headers:

    Content-Type: application/json

Payload:
```json
{
  "email":"john.smith@gmail.com",
  "password":"password123"
}
```

#### Response:

Header:

    Content-Type: application/json
    Authorization: Bearer <JWT TOKEN>

Payload:
```json
{
  "id": "f36a81af-11d5-4c55-acd7-edb00f69ba0a",
  "profilePicture": "/avatar/Avatar-10.png",
  "username": "john",
  "email": "john.smith@gmail.com",
  "phone": "+35059118440",
  "role": "Player",
  "isActive": true,
  "createdDate": "2022-04-07T15:20:18.925852",
  "updatedDate": "2022-04-07T15:20:18.925852"
}
```

### Get Account Details

#### Request

Method: GET

URL: http://localhost/users/me

Headers:

    Authorization: Bearer <JWT TOKEN>

Payload: None

#### Response:

Header:

    Content-Type: application/json
    Authorization: Bearer <JWT TOKEN>

Payload:
```json
{
  "id": "f36a81af-11d5-4c55-acd7-edb00f69ba0a",
  "profilePicture": "/avatar/Avatar-10.png",
  "username": "john",
  "email": "john.smith@gmail.com",
  "phone": "+35059118440",
  "role": "Player",
  "isActive": true,
  "createdDate": "2022-04-07T15:20:18.925852",
  "updatedDate": "2022-04-07T15:20:18.925852"
}
```
