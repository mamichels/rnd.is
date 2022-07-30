# rnd.is
A very small project that provides an HTTP API for random numbers.

## Usage
This API exposes the following endpoints:

## /number
```shell
GET /number?min=<LOWER_BOUND>&max=<UPPER_BOUND>
```
Example:
```shell
curl -X GET 'localhost:8080/number?min=1&max=404'
```