# Pulsar admin

[![Build Status](https://travis-ci.org/miton18/pulsar-cli.svg?branch=master)](https://travis-ci.org/miton18/pulsar-cli)

Utilities (CLI + lib) to manager a Pulsar instance with admin API.

[More informations about Pulsar](https://pulsar.apache.org/)

[More informations about Admin API](https://pulsar.apache.org/admin-rest-api/)


## To do

| API | Support |
|--|--|
| [Bookies](https://pulsar.apache.org/admin-rest-api/#tag/bookies) | [] |
| [Broker-stats](https://pulsar.apache.org/admin-rest-api/#tag/broker-stats) | [] |
| [Brokers](https://pulsar.apache.org/admin-rest-api/#tag/brokers) | [] |
| [Clusters](https://pulsar.apache.org/admin-rest-api/#tag/clusters) | [] |
| [Namespaces](https://pulsar.apache.org/admin-rest-api/#tag/namespaces) | [] |
| [Non persistent topics](https://pulsar.apache.org/admin-rest-api/#tag/non-persistent-topic) | [] |
| [Persistent topics](https://pulsar.apache.org/admin-rest-api/#tag/persistent-topic) | [] |
| [Resouce quotas](https://pulsar.apache.org/admin-rest-api/#tag/resource-quotas) | [] |
| [Schemas](https://pulsar.apache.org/admin-rest-api/#tag/schemas) | [] |
| [Tenants](https://pulsar.apache.org/admin-rest-api/#tag/tenants) | [x] |

## Usage

Use `pulsar-admin --help` to discover CLI possibilities

## Dev

```sh
cargo build
```
