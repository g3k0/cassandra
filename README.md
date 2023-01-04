# Cassandra

## Overview

Cassandra is a comprehensive trading bot. <br />
Since the most popular open source trading bots are focused on crypto exchange only, the purpose of this bot is primarily to give the opportunity to do automated trading on many different financial assets. <br />

## Goal 
The goal of this project is primarily to build a bot able to operate in different exchange marketplaces, defined a trading strategy. <br />
But the medium-long term goal is more ambitious: Cassandra wants to be able to query the data from third-parties systems and elaborate trading strategies that presumably will lead to a profit for the trader.

The feture that will be implemented in the short-medium period are:

* multiple brokers integration;
* a configuration of the strategy;
* a trade historian; 
* a UI interface;
* integration with Telegram bot;

## Technology
Cassandra is written in Rust, because: 
* Rust is fast, robust and its software is safe;
* Rust is very performant because of its memory management;
* Rust supports concurrent programming;
* Rust executables can be platform independent;
* I like Rust;

For development purposes Alpaca trading API are used. It is possible to find the documentation about Alpaca inside the exchage_api library. 
## External depedencies 
* libssl-dev (debian-based OS)

## Lincense

MIT

## NOTE
Since that the goal is ambitious and there is little free time to work on it, this project is open to anyone who wants to collaborate.