# Exchange API

## Overview
This Rust library contains the logic that performs http requests to different supported brokers in order to make market exchanges and in general to interact with the broker.<br /> 
The library is an abstraction layer that wraps each single broker API SDK.

## Supported brokers
* [Alpaca](https://alpaca.markets/)


### Alpaca
Available resources:
 * [Trading API](https://alpaca.markets/docs/api-references/trading-api/)
 * [Postman Collections](https://github.com/alpacahq/alpaca-postman)

In order to use the Alpaca API, you need to register to the web platform, choose a demo account or a real account and get the base url, api kei id and api key secret related to the account chosen. <br />
Set the following env variables at system level before to use the api:

```bash
APCA_API_BASE_URL
APCA_API_KEY_ID
APCA_API_SECRET_KEY
```
