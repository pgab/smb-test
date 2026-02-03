# smb-test

When using `smb` crate, version 0.11.1, with windows, sometime the following error occurs:

```
Error: TransportError(IoError(Os { code: 10057, kind: NotConnected, message: "Eine Anforderung zum Senden oder Empfangen von Daten wurde verhindert, da der Socket nicht verbunden ist und (beim Senden über einen Datagrammsocket mit einem sendto-Aufruf) keine Adresse angegeben wurde." }))
```

This comes up with the multi-threaded runtime. The async runtime with tokio works fine.

This repository resembles the environment in which the error seems occur. The application needs to open several clients consecutively. There is no defined number when error occurs, sometimes after two runs, sometimes after 19, adding a timeout doesn't help neither and not even closing the connection.

# Testing

## Docker

```
docker compose up [-d]
```

To obtain the IP address for the docker container you may run

```
wsl hostname -I
```

This will print out a list of IPs. In my case the first option worked well.

```
wsl hostname -i
```

Did not work for me (lower case `i` vs upper case `i` in the former example).

Also running

```
ip route show | grep -i default | awk '{ print $3}'
```

inside the default distro did not work for me, neither did the result of

```
cat /etc/resolv.conf
```

## Run Tests

Adjust the IPs accordingly in `src/lib.rs`.

```
cargo test
```