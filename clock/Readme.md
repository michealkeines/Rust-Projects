# Atomic Clock

Atomic Clock provdies options to get and set precise local time using system call and also syncs the current time using NTP protocol

## Features

-> SET time using OS specific system call, curently supports windows, linux

-> SYNC time using manually requests to NTP service provided by big servers, and update the local time automatically

-> GET different formatted time standardsc currently supports rfc2822, rfc3339


## Run

```
cargo run -- <args>
```