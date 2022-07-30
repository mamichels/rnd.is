# [![rnd-is-logo-rclear.png](https://i.postimg.cc/5t7GCYH1/rnd-is-logo-rclear.png)](https://postimg.cc/NyXph07P)
![licence](https://img.shields.io/github/license/mamichels/rnd.is?style=flat-square)

A very small project that provides an HTTP API for random numbers.

## Usage
Use the publicly available instance or run your own instance.

This API exposes the following endpoints:

## /number
```shell
GET /number?min=<LOWER_BOUND>&max=<UPPER_BOUND>
```
Example:
```shell
curl -X GET 'rnd.is/number?min=1&max=404'
```