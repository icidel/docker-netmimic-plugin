# docker-netmimic-plugin
A network simulator plugin for Docker's Libnetwork

This project is in incubation (aka pre-pre-pre-alpha !) phase. Fell free to participate by communicating with me through pull requests or issues.

[The docker engine can be extended with volume, network or authentication plugins|https://docs.docker.com/engine/extend/legacy_plugins/]. For this project we want to create a network simulator plugin.

The NetMimic project goals are:
1. act as a tunnel (TUN/TAP) interface to record network conversations between the docker instance and distant servers
2. allow to switch in "mimic" mode for specified distant servers:
  * do not send messages anymore to those servers
  * use a response generator to return responses
3. allow for on-demand changes to the simulated network behavior:
  * change the response time
  * simulate faults

The Rust language has been chosen for this task because:
- it avoid copying the data if it is not strictly required
- it is designed to be both fast and reliable
- it includes compilation check for data owning and borrowing, effectively avoiding common memory pitfalls
- to build a Rust program, you use the "cargo build" command, which is quite cool for a docker plugin :-)
