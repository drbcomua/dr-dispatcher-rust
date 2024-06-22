Build Docker
---

```shell
docker build --no-cache -t dispatcher-rust .
```

Run
---

```shell
docker run -p 8080:8080 dispatcher-rust
```
