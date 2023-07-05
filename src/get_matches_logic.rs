use crate::structs_list::*;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::Utc;
use chrono_tz::America::Toronto;
use itertools::Itertools;

use std::error::Error;

use reqwest::{Client, Request};

pub fn today_day() -> String {
    chrono::Local::now()
        .with_timezone(&Toronto)
        .format("%d/%m/%Y")
        .to_string()
}

pub fn fmt_match_array(matches_resp: &[Event]) -> std::string::String {
    if matches_resp.is_empty() {
        "No matches found".to_string()
    } else {
        let fmt_match_array: String = matches_resp
            .iter()
            .format_with("\n", |tennis, f| f(&format_args!("{}", tennis)))
            .to_string();
        fmt_match_array
    }
}

pub async fn send_schedule(
    api_key: &str,
    client: &Client,
    live_or_sched: &str,
) -> Result<String, Box<dyn Error>> {
    let mut url: String = "init".to_string();
    const SCHED_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/category/3/events/";
    const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

    let dt_for_api: DelayedFormat<StrftimeItems> = chrono::Local::now()
        .with_timezone(&Toronto)
        .format("%d/%-m/%Y");

    if live_or_sched == "live" {
        url = format!("{}?rapidapi-key={}", LIVE_URL, api_key);
    } else if live_or_sched == "upcoming" {
        url = format!(
            "{}{}?rapidapi-key={}",
            SCHED_URL,
            dt_for_api.to_string().trim(),
            api_key
        );
    }

    let request: Request = client.get(url).build().unwrap();
    let mut resp: Root = client.execute(request).await?.json::<Root>().await?;

    let resp_to_send: &mut Vec<Event> = &mut resp.events;

    // keep only matches that start today but are yet to start in the vec
    if live_or_sched == "upcoming" {
        resp_to_send.retain(|game| {
            game.status.type_field == "notstarted" && today_day() == game.day_mnth_yr()
        });
    }
    if live_or_sched == "live" {
        resp_to_send.retain(|game| {
            game.tournament.category.id.unwrap_or_default() == 3
                && game.home_team.country.alpha2.is_some()
                && game.away_team.country.alpha2.is_some()
        })
    }

    let match_results: String = fmt_match_array(resp_to_send);
    Ok(match_results)
}

pub async fn player_search(
    player: &str,
    api_key: &str,
    client: &Client,
) -> Result<String, Box<dyn Error>> {
    let url: String = format!(
        "https://tennisapi1.p.rapidapi.com/api/tennis/search/{}?rapidapi-key={}",
        player, api_key
    );
    let request: Request = client.get(url).build()?;
    let resp: PlayerResults = client
        .execute(request)
        .await?
        .json::<PlayerResults>()
        .await?;

    // J.J. Wolf is weird and has two ids, one doesn't work, `398806`, but is the first result when searching `jj wolf`, this forces working ID
    let call_matches: &str = {
        let mut ids = resp.results[0].entity.id;
        let player_name = &resp.results[0].entity.name;
        if ids == 398806 {
            ids = 210479;
        }
        &get_player_matches(ids, api_key, client)
            .await
            .unwrap_or(format!(
                "Could not find upcoming matches for {:?}",
                player_name
            ))
    };

    Ok(call_matches.to_string())
}

pub async fn get_player_matches(
    player_id: i64,
    api_key: &str,
    client: &Client,
) -> Result<String, Box<dyn Error>> {
    let url: String = format!(
        "https://tennisapi1.p.rapidapi.com/api/tennis/player/{}/events/next/0?rapidapi-key={}",
        player_id, api_key
    );
    let request: Request = client.get(url).build().unwrap();
    let resp: Root = client
        .execute(request)
        .await
        .unwrap_or_else(|error| panic!("ERROR!:{:?}", error))
        .json::<Root>()
        .await?;
    let first_event = &resp.events[0];
    let match_to_return = match first_event.tournament.slug.contains("doubles")
        | first_event.tournament.slug.contains("qualifying")
    {
        true => &resp.events[1],
        false => first_event,
    };

    Ok(match_to_return.to_string())
}
