# Tennis_Bot_2
electric boogaloo<br>
requires `docker` and `docker compose`, or, `cargo` options alone should be fine to run outside of docker<br>
works with https://rapidapi.com/fluis.lacasse/api/tennisapi1<br>
copy .env.example to .env, fill fields, `docker compose up --build`
Change to local timezone in the `Dockerfile` for accturate match days/times, TZ defaults to GMT when unset<br>
be sure to change the `chrono_tz` import and use on line 384 in `get_matches_logic.rs` as well
