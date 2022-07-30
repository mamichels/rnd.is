# [![rnd-is-logo-rclear.png](https://i.postimg.cc/5t7GCYH1/rnd-is-logo-rclear.png)](https://rnd.is/)
[![licence](https://img.shields.io/github/license/mamichels/rnd.is?style=flat-square)](https://github.com/mamichels/rnd.is/blob/main/LICENSE)
[![status](https://img.shields.io/website?down_color=red&down_message=Down&label=RND.IS&style=flat-square&up_color=green&up_message=Live&url=http%3A%2F%2Frnd.is)](https://rnd.is/)
[![openapi](https://img.shields.io/badge/OpenAPI-Schema-green)](https://rnd.is/openapi)

A lightweight HTTP API that provides random numbers.

## Usage
Use the publicly available instance or run your own instance.

## API

### /number
Returns a random number between the lower and upper bound.
```shell

GET /number?min=<LOWER_BOUND>&max=<UPPER_BOUND>

# E.g.:
curl -X GET 'rnd.is/number?min=1&max=404'
```
Returns:
```json
{
  "apiVersion": "0.2.0",
  "params": {
    "min": 1,
    "max": 404
  },
  "data": {
    "kind": "number",
    "value": 327
  }
}
```
### /numbers
Returns an array of random numbers between the lower and upper bound.
```shell

GET /number?length=<LENGTH>&min=<LOWER_BOUND>&max=<UPPER_BOUND>

# E.g.:
curl -X GET 'rnd.is/numbers?length=3&min=1&max=404'
```
Returns:
```json
{
  "apiVersion": "0.2.0",
  "params": {
    "min": 1,
    "max": 404,
    "length": 3
  },
  "data": {
    "kind": "number",
    "value": [
      304,
      139,
      56
    ]
  }
}
```
