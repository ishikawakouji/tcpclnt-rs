# tcpclnt-rs

This program is sample code to send some messages via TCP. It is written with Rust.

tcpclnt-rs reads messages from std I/O, then you can use redirection like;

```
tcpclnt-rs < hoge.txt
```

## Usage
tcpclnt-rs can understand one argument, connection IP address and port number.

```
tcpclnt-rs 127.0.0.1:1000
```

If there is no argument, it will connect to 127.0.0.1:1000. Or you can customize to any.