<<<<<<< HEAD
use crate::structs_list::*;
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};
||||||| 9b07b57
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};
=======
use crate::structs_list::*;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::Utc;
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
use chrono_tz::America::Toronto;
use itertools::Itertools;

use std::error::Error;

use reqwest::{Client, Request};
<<<<<<< HEAD

pub fn get_today() -> chrono::DateTime<chrono::Local> {
||||||| 9b07b57
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    // pub away_score: Score,
    pub away_team: Team,
    // pub changes: Changes,
    // pub custom_id: String,
    // pub final_result_only: bool,
    // pub first_to_serve: Option<i64>,
    // pub has_global_highlights: bool,
    //pub home_score: Score,
    pub home_team: Team,
    pub id: Option<i64>,
    // pub last_period: Option<String>,
    //  pub periods: Periods,
    // pub round_info: Option<RoundInfo>,
    // pub slug: String,
    pub start_timestamp: Option<i64>,
    pub status: Status,
    pub time: Option<Time>,
    pub tournament: Tournament,
    // pub winner_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    // pub disabled: Option<Value>,
    pub id: Option<i64>,
    pub name: String,
    pub short_name: String,
    pub slug: String,
    //   pub sport: Sport,
    //   pub sub_teams: Option<Vec<SubTeam>>,
    //   pub team_colors: TeamColors2,
    // #[serde(rename = "type")]
    // pub type_field: Option<i64>,
    // pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    // pub code: Option<i64>,
    // pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub current_period_start_timestamp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub category: Category,
    // pub id: Option<i64>,
    pub name: String,
    // pub priority: Option<i64>,
    pub slug: String,
    // pub unique_tournament: UniqueTournament,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    // pub flag: String,
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    //  pub sport: Sport5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueTournament {
    // pub category: Category2,
    // pub has_event_player_statistics: bool,
    // pub has_position_graph: Option<String>,
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    // pub user_count: Option<i64>,
}

// find player matches by ID

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResults {
    pub results: Vec<Player>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub entity: PlayerDetails,
    pub score: f64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDetails {
    // pub country: Country,
    // pub disabled: Option<bool>,
    // pub gender: Option<String>,
    pub id: i64,
    pub name: String,
    pub name_code: String,
    // pub national: bool,
    // pub ranking: Option<i64>,
    pub short_name: String,
    pub slug: String,
    // pub sport: Sport,
    // pub team_colors: TeamColors,
    // #[serde(rename = "type")]
    // pub type_field: i64,
    // pub user_count: i64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Country {
//     pub alpha2: Option<String>,
//     pub name: Option<String>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport {
//     pub id: i64,
//     pub name: String,
//     pub slug: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TeamColors {
//     pub primary: String,
//     pub secondary: String,
//     pub text: String,
// }

impl fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TennisMatch<'a> {
    pub home_team_name: &'a str,
    pub away_team_name: &'a str,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveTennisMatch<'a> {
    pub home_team_name: &'a str,
    pub away_team_name: &'a str,
}

impl fmt::Display for TennisMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs. {} // {}",
            self.home_team_name, self.away_team_name, self.time
        )
    }
}

impl fmt::Display for LiveTennisMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs. {}", self.home_team_name, self.away_team_name,)
    }
}

#[derive(Debug)]
pub struct CouldNotFindPlayer {
    name: String,
}

impl fmt::Display for CouldNotFindPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find '{}'", self.name)
    }
}

pub fn get_today() -> chrono::DateTime<chrono::Local> {
=======

pub fn today_day() -> String {
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
    chrono::Local::now()
<<<<<<< HEAD
}
pub fn time_builder(event: &Event) -> chrono::DateTime<chrono_tz::Tz> {
    let binding = event.time.as_ref();
    let time = binding.unwrap().current_period_start_timestamp;
    let time_to_return = if let Some(time) = time {
        NaiveDateTime::from_timestamp_opt(time, 0).expect("Match missing time4")
    } else {
        NaiveDateTime::from_timestamp_opt(event.start_timestamp.expect("Match missing time5"), 0)
            .expect("Match missing time6")
    };
    time_to_return
        .and_local_timezone(Utc)
        .unwrap()
        .with_timezone(&Toronto)
||||||| 9b07b57
}
pub fn time_builder(event: Event) -> chrono::NaiveDateTime {
    let event_time = event
        .time
        .expect("Match missing time2")
        .current_period_start_timestamp;
    if event_time.as_ref().is_some() {
        NaiveDateTime::from_timestamp_opt(event_time.expect("Match missing time3"), 0)
            .expect("Match missing time4")
    } else {
        NaiveDateTime::from_timestamp_opt(event.start_timestamp.expect("Match missing time5"), 0)
            .expect("Match missing time6")
    }
=======
        .with_timezone(&Toronto)
        .format("%d/%m/%Y")
        .to_string()
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
}

<<<<<<< HEAD
pub fn fmt_match_array(matches_resp: &Vec<TennisMatch>) -> std::string::String {
||||||| 9b07b57
pub fn fmt_match_array(matches_resp: Vec<TennisMatch>) -> std::string::String {
=======
pub fn fmt_match_array(matches_resp: &[Event]) -> std::string::String {
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
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

<<<<<<< HEAD
pub fn fmt_live_match_array(matches_resp: &Vec<LiveTennisMatch>) -> std::string::String {
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
    let match_array: &mut Vec<TennisMatch> = &mut Vec::new();
    let today_day = get_today().format("%d/%m/%Y").to_string();
    //    if root.iter().all(|team| team)
    root.iter().for_each(|team| {
        if &team.status.type_field == "notstarted" {
            let event_day = time_builder(team);

            if today_day == event_day.format("%d/%m/%Y").to_string() {
                {
                    let mut final_time = event_day.format("%l:%M %p %Z").to_string();
                    if team.tournament.name.to_lowercase().contains("qualifying") {
                        final_time += " (qualifying)"
                    }
                    let match_builder = TennisMatch {
                        home_team_name: &team.home_team.name,
                        away_team_name: &team.away_team.name,
                        time: final_time.to_string(),
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

    fmt_live_match_array(&match_array)
}

pub async fn send_live(
||||||| 9b07b57
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

pub fn get_todays_matches(root: Vec<Event>) -> std::string::String {
    // consider iter
    let mut match_array: Vec<TennisMatch> = Vec::new();
    let today_day = get_today().format("%d/%m/%Y").to_string();
    for team in root.iter() {
        if team.tournament.category.name == "ATP" && team.status.type_field == "notstarted" {
            let event_day = time_builder(team.clone())
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
    }
    fmt_match_array(match_array)
}

pub fn get_live_matches(root: Vec<Event>) -> std::string::String {
    // consider iter
    let mut match_array: Vec<LiveTennisMatch> = Vec::new();
    for team in root.iter() {
        if team.tournament.category.name == "ATP" {
            let match_builder: LiveTennisMatch = LiveTennisMatch {
                home_team_name: &team.home_team.name,
                away_team_name: &team.away_team.name,
            };

            match_array.push(match_builder)
        }
    }

    fmt_live_match_array(match_array)
}

pub async fn send_live(
=======
pub async fn send_schedule(
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
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

<<<<<<< HEAD
    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_live_matches(&resp.events);
    Ok(match_results)
}
||||||| 9b07b57
    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_live_matches(resp.events);
    Ok(match_results)
}
=======
    let resp_to_send: &mut Vec<Event> = &mut resp.events;
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a

<<<<<<< HEAD
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
||||||| 9b07b57
pub async fn send_today_schedule(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let dt_for_api: DelayedFormat<StrftimeItems> = get_today().format("%d/%-m/%Y");
    const SCHED_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/";
    let url: String = format!(
        "{}{}?rapidapi-key={}",
        SCHED_URL,
        dt_for_api.to_string().trim(),
        api_key
    );
    let request: Request = client.get(url).build().unwrap();
    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_todays_matches(resp.events);
=======
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
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
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

<<<<<<< HEAD
    let match_builder: TennisMatch = TennisMatch {
        home_team_name: match_to_return.home_team.name.as_str(),
        away_team_name: match_to_return.away_team.name.as_str(),
        time: time_builder(match_to_return)
            .format("%B %e,%l:%M %p %Z")
            .to_string(),
    };

    Ok(match_builder.to_string())
||||||| 9b07b57
    let time_stamp = time_builder(match_to_return.clone());
    let final_time = time_stamp
        .and_local_timezone(Utc)
        .unwrap()
        .with_timezone(&Toronto)
        .format("%B %e,%l:%M %p %Z")
        .to_string();

    let match_builder: TennisMatch = TennisMatch {
        home_team_name: &match_to_return.home_team.name,
        away_team_name: &match_to_return.away_team.name,
        time: final_time,
    };

    Ok(match_builder.to_string())
=======
    Ok(match_to_return.to_string())
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a
}
