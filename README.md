Build:

```bash
docker buildx build -f .lenra/Dockerfile  --load -t lenra/my-app --cache-to=type=local,dest==.lenra/dockercache --cache-from=type=local,src=.lenra/dockercache .
```

```bash
lenra build
lenra start --attach none
lenra stop
``` 