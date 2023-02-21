use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};
use chrono_tz::America::Toronto;
use itertools::Itertools;
use reqwest::{Client, Request};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
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
    pub custom_id: String,
    pub final_result_only: bool,
    pub first_to_serve: Option<i64>,
    pub has_global_highlights: bool,
    //pub home_score: Score,
    pub home_team: Team,
    pub id: Option<i64>,
    pub last_period: Option<String>,
    //  pub periods: Periods,
    // pub round_info: Option<RoundInfo>,
    pub slug: String,
    pub start_timestamp: Option<i64>,
    pub status: Status,
    pub time: Time,
    pub tournament: Tournament,
    pub winner_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub disabled: Option<Value>,
    pub id: Option<i64>,
    pub name: String,
    pub short_name: String,
    pub slug: String,
    //   pub sport: Sport,
    //   pub sub_teams: Option<Vec<SubTeam>>,
    //   pub team_colors: TeamColors2,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub code: Option<i64>,
    pub description: String,
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
    pub id: Option<i64>,
    pub name: String,
    pub priority: Option<i64>,
    pub slug: String,
    pub unique_tournament: UniqueTournament,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub flag: String,
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    //  pub sport: Sport5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueTournament {
    // pub category: Category2,
    pub has_event_player_statistics: bool,
    pub has_position_graph: Option<String>,
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub user_count: Option<i64>,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TennisMatch {
    pub home_team_name: String,
    pub away_team_name: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveTennisMatch {
    pub home_team_name: String,
    pub away_team_name: String,
}

impl fmt::Display for TennisMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs. {} // {}",
            self.home_team_name, self.away_team_name, self.time
        )
    }
}

impl fmt::Display for LiveTennisMatch {
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
        write!(f, "Placeholder 1 '{}'", self.name)
    }
}

pub fn get_today() -> chrono::DateTime<chrono::Local> {
    let date = chrono::Local::now();
    date
}
pub fn time_builder(event: Event) -> chrono::NaiveDateTime {
    let time_stamp = if event.time.current_period_start_timestamp.is_some() {
        NaiveDateTime::from_timestamp_opt(event.time.current_period_start_timestamp.unwrap(), 0)
            .unwrap()
    } else {
        NaiveDateTime::from_timestamp_opt(event.start_timestamp.unwrap(), 0).unwrap()
    };

    time_stamp
}

pub fn get_todays_matches(root: Vec<Event>) -> std::string::String {
    // consider iter
    let mut match_array: Vec<TennisMatch> = Vec::new();
    let today_day = get_today().format("%d/%m/%Y").to_string();
    for team in root {
        if team.tournament.category.name == "ATP" && team.status.type_field == "notstarted" {
            let event_day = time_builder(team.clone()).format("%d/%m/%Y").to_string();
            if today_day == event_day {
                {
                    let time_stamp = time_builder(team.clone());
                    let final_time = time_stamp
                        .and_local_timezone(Utc)
                        .unwrap()
                        .with_timezone(&Toronto)
                        .format("%l:%M %p %Z")
                        .to_string();
                    let match_builder: TennisMatch = TennisMatch {
                        home_team_name: team.home_team.name,
                        away_team_name: team.away_team.name,
                        time: final_time,
                    };

                    match_array.push(match_builder)
                }
            }
        }
    }
    if match_array.is_empty() {
        return "No matches found".to_string();
    } else {
        let fmt_match_array: String = match_array
            .iter()
            .format_with("\n", |tennis, f| f(&format_args!("{}", tennis)))
            .to_string();
        fmt_match_array
    }
}

pub fn get_live_matches(root: Vec<Event>) -> std::string::String {
    // consider iter
    let mut match_array: Vec<LiveTennisMatch> = Vec::new();
    for team in root {
        if team.tournament.category.name == "ATP" {
            let match_builder: LiveTennisMatch = LiveTennisMatch {
                home_team_name: team.home_team.name,
                away_team_name: team.away_team.name,
            };

            match_array.push(match_builder)
        }
    }

    if match_array.is_empty() {
        return "No matches found".to_string();
    } else {
        let fmt_match_array: String = match_array
            .iter()
            .format_with("\n", |tennis, f| f(&format_args!("{}", tennis)))
            .to_string();
        fmt_match_array
    }
}

pub async fn send_live(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

    let url: String = format!("{}?rapidapi-key={}", LIVE_URL, api_key);

    let request: Request = client.get(url).build().unwrap();

    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_live_matches(resp.events);

    Ok(match_results)
}

pub async fn send_today_schedule(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let dt_for_api: DelayedFormat<StrftimeItems> = get_today().format("%d/%m/%Y");
    const SCHED_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/";
    let url: String = format!("{}{}?rapidapi-key={}", SCHED_URL, dt_for_api, api_key);
    let request: Request = client.get(url).build().unwrap();
    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_todays_matches(resp.events);

    Ok(match_results)
}
