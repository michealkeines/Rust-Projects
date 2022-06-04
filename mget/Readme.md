# mget

Simple application to resolve hostname using build-in relsover and make http requests to the server, this helps to understand DNS, HTTP protocals implementation in depth.

## Features:

DNS - makes manual DNS query and resolves the hostname, custome dns server could be specified

Interface - custom TAP interface support

HTTP - Assumbles sockect and http request and responses manually

## setup

```
./setup.sh

cargo run -- <TAP> <hostname>

```