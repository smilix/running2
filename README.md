# running2 
Logs my running activities. Successor of [running](https://github.com/smilix/running). 

## Build

```shell
cargo build --release
```

static version
```shell
# needs: 
# sudo apt-get install musl-tools
cargo build --release --target x86_64-unknown-linux-musl
```

## Run

```shell
copy example.env .env
# edit the settings
./running2
```

Tip: Use the systemd init file in `extras/`

## Testing requests
```bash
# list runs
curl -v --header "Content-Type:application/json" 'localhost:8080/runs?max=5'

# add new run
curl -vX POST --header "Content-Type:application/json" -d @new.json localhost:8080/runs

# update run
curl -vX PUT --header "Content-Type:application/json" -d @update.json localhost:8080/runs/1

# delete run
curl -vX DELETE --header "Content-Type:application/json" localhost:8080/runs/
```
