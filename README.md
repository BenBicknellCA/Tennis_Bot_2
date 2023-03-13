Tennis_Bot_2

electric boogaloo

requires `docker` and `docker compose`, or, `cargo` options alone should be fine to run outside of `docker`

works with https://rapidapi.com/fluis.lacasse/api/tennisapi1

copy `.env.example` to `.env`, fill fields, `docker compose up --build`

change to local timezone in the `Dockerfile` for accurate match day/times, TZ defaults to GMT when unset

be sure to change the `chrono_tz` import and use on line 384 in `get_matches_logic.rs` as well



if you want to compile outside of the docker container because it’s going slow, you can replace the `Dockerfile` in the Repo with the one below,
and run by executing the script, ensure the script file is executable with `chmod u+x`.
if you don't want to deploy to another machine, you can get rid of the rsync command and `TARGET_HOST`, `TARGET_PATH`, and
replace the SSH command with `cd $TARGET_PATH; docker compose up --build -d`
``` bash
#!/bin/bash
set  -o  errexit
set  -o  nounset
set  -o  pipefail
set  -o  xtrace

readonly  TARGET_HOST=user@ip
readonly  TARGET_PATH= :' where you want the binary to be copied to on the machine you
want to host the bot'

# readonly TARGET_ARCH=  use this if you want to compile for an architechture different than the
# machine you are using to compile
readonly  SOURCE_PATH=./target/release/tennis_bot # where the binary will compile to 
# before it is copied to the machine you are compiling for

cargo build  --release  # --target=${TARGET_ARCH} uncomment if you set TARGET_ARCH
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}/ # copies the binary to the directory specified on the machine
# you will be hosting the bot from. you'll need the rsync package

ssh -t ${TARGET_HOST} "cd $TARGET_PATH; docker compose up --build -d"
# you will need SSH access to the target machine, this command connects, starts the container
# and disconnects
```

``` dockerfile
FROM ubuntu:latest
RUN apt-get update && apt-get -y install ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY ./tennis_bot /app/

CMD [ “/app/tennis_bot”]
```
