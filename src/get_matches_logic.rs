use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};
use chrono_tz::America::Toronto;
use itertools::Itertools;
use reqwest::{Client, Request};
use crate::structs_list::*;


pub fn get_today() -> chrono::DateTime<chrono::Local> {
    chrono::Local::now()
}
pub fn time_builder(event: &Event) -> chrono::NaiveDateTime {
    let binding = event.time.as_ref();
    let time = binding.unwrap().current_period_start_timestamp;
    if let Some(time) = time {

        NaiveDateTime::from_timestamp_opt(time, 0)
            .expect("Match missing time4")

    }
    

     else {
        NaiveDateTime::from_timestamp_opt(event.start_timestamp.expect("Match missing time5"), 0)
            .expect("Match missing time6")
    }
}

pub fn fmt_match_array(matches_resp: Vec<TennisMatch>) -> std::string::String {
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

pub fn fmt_live_match_array(matches_resp: Vec<LiveTennisMatch>) -> std::string::String {
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

pub fn get_todays_matches(root: &[Event]) -> std::string::String {
    // consider iter
    let mut match_array: Vec<TennisMatch> = Vec::new();
    let today_day = get_today().format("%d/%m/%Y").to_string();
//    if root.iter().all(|team| team)
    root.iter().for_each(|team| {
         if team.status.type_field == "notstarted" {
            let event_day = time_builder(team)
                .and_local_timezone(Utc)
                .unwrap()
                .with_timezone(&Toronto);

            if today_day == event_day.format("%d/%m/%Y").to_string() {
                {
                    let mut final_time = event_day.format("%l:%M %p %Z").to_string();
                    if team.tournament.name.to_lowercase().contains("qualifying") {
                        final_time += " (qualifying)"
                    }
                    let match_builder: TennisMatch = TennisMatch {
                        home_team_name: &team.home_team.name,
                        away_team_name: &team.away_team.name,
                        time: final_time,
                    };

                    match_array.push(match_builder)
                }
            }
        }
    });
    fmt_match_array(match_array)
}

pub fn get_live_matches(root: &[Event]) -> std::string::String {
    // consider iter
    let mut match_array: Vec<LiveTennisMatch> = Vec::new();
    root.iter().for_each(|team| {
        if team.tournament.category.name == "ATP" {
            let match_builder: LiveTennisMatch = LiveTennisMatch {
                home_team_name: &team.home_team.name,
                away_team_name: &team.away_team.name,
            };

            match_array.push(match_builder)
        }
    });

    fmt_live_match_array(match_array)
}

pub async fn send_live(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

    let url: String = format!("{}?rapidapi-key={}", LIVE_URL, api_key);

    let request: Request = client.get(url).build().unwrap();

    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_live_matches(&resp.events);
    Ok(match_results)
}

pub async fn send_today_schedule(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let dt_for_api: DelayedFormat<StrftimeItems> = get_today().format("%d/%-m/%Y");
    const SCHED_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/category/3/events/";
    let url: String = format!(
        "{}{}?rapidapi-key={}",
        SCHED_URL,
        dt_for_api.to_string().trim(),
        api_key
    );
    let request: Request = client.get(url).build().unwrap();
    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_todays_matches(&resp.events);
    Ok(match_results)
}

pub async fn player_search(
    player: &str,
    api_key: &str,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
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
    let call_matches: String = {
        let mut ids = resp.results[0].entity.id;
        let player_name = &resp.results[0].entity.name;
        if ids == 398806 { ids = 210479;}
        get_player_matches(ids, api_key, client).await.unwrap_or(format!("Could not find upcoming matches for {:?}", player_name))
    };

    Ok(call_matches)
}

pub async fn get_player_matches(
    player_id: i64,
    api_key: &str,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!(
        "https://tennisapi1.p.rapidapi.com/api/tennis/player/{}/events/next/0?rapidapi-key={}",
        player_id, api_key
    );
    let request: Request = client.get(url).build().unwrap();
    let resp: Root = client.execute(request).await.unwrap_or_else(|error|panic!("ERROR!:{:?}", error)).json::<Root>().await?;
    let first_event = &resp.events[0];
    let match_to_return = match first_event.tournament.slug.contains("doubles")
            | first_event.tournament.slug.contains("qualifying") {
        true => &resp.events[1],
        false => first_event,
    };



    let final_time = time_builder(match_to_return)
        .and_local_timezone(Utc)
        .unwrap()
        .with_timezone(&Toronto)
        .format("%B %e,%l:%M %p %Z")
        .to_string();

    let match_builder: TennisMatch = TennisMatch {
        home_team_name: match_to_return.home_team.name.as_str(),
        away_team_name: match_to_return.away_team.name.as_str(),
        time: final_time,
    };

    Ok(match_builder.to_string())
}
