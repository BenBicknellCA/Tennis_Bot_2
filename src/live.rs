extern crate serde;
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
    TimeZone, Utc,
};
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
    pub away_score: Score,
    pub away_team: Team,
    pub changes: Changes,
    pub custom_id: String,
    pub final_result_only: bool,
    pub first_to_serve: Option<i64>,
    pub has_global_highlights: bool,
    pub home_score: Score,
    pub home_team: Team,
    pub id: Option<i64>,
    pub last_period: Option<String>,
    pub periods: Periods,
    pub round_info: Option<RoundInfo>,
    pub slug: String,
    pub start_timestamp: Option<i64>,
    pub status: Status,
    pub time: Time,
    pub tournament: Tournament,
    pub winner_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub current: Option<i64>,
    pub display: Option<i64>,
    pub normaltime: Option<Value>,
    pub period1: Option<i64>,
    #[serde(rename = "period1TieBreak")]
    pub period1tie_break: Option<i64>,
    pub period2: Option<i64>,
    #[serde(rename = "period2TieBreak")]
    pub period2tie_break: Option<i64>,
    pub period3: Option<i64>,
    #[serde(rename = "period3TieBreak")]
    pub period3tie_break: Option<i64>,
    pub period4: Option<i64>,
    pub period5: Option<i64>,
    #[serde(rename = "period5TieBreak")]
    pub period5tie_break: Option<i64>,
    pub point: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub disabled: Option<Value>,
    pub id: Option<i64>,
    pub name: String,
    pub short_name: String,
    pub slug: String,
    pub sport: Sport,
    pub sub_teams: Option<Vec<SubTeam>>,
    pub team_colors: TeamColors2,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubTeam {
    pub disabled: Option<Value>,
    pub gender: Option<String>,
    pub id: Option<i64>,
    pub name: String,
    pub name_code: Option<String>,
    pub national: Option<Value>,
    pub ranking: Option<i64>,
    pub short_name: String,
    pub slug: String,
    pub sport: Sport2,
    pub team_colors: TeamColors,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport2 {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamColors {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamColors2 {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changes {
    pub change_timestamp: Option<i64>,
    pub changes: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport3 {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubTeam2 {
    pub disabled: Value,
    pub gender: String,
    pub id: Option<i64>,
    pub name: String,
    pub name_code: String,
    pub national: bool,
    pub ranking: Option<i64>,
    pub short_name: String,
    pub slug: String,
    pub sport: Sport4,
    pub sub_teams: Option<Vec<Value>>,
    pub team_colors: TeamColors3,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport4 {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamColors3 {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamColors4 {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Periods {
    pub current: String,
    pub period1: String,
    pub period2: String,
    pub period3: String,
    pub period4: String,
    pub period5: String,
    pub point: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundInfo {
    pub cup_round_type: Option<i64>,
    pub name: String,
    pub round: Option<i64>,
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
    pub sport: Sport5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport5 {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueTournament {
    pub category: Category2,
    pub has_event_player_statistics: bool,
    pub has_position_graph: Option<String>,
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    pub flag: String,
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub sport: Sport6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport6 {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub has_position_graph: Option<String>,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct TennisMatch {
    pub home_team_name: String,
    pub away_team_name: String,
    pub time: String,
}

impl fmt::Display for TennisMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs. {} / {}",
            self.home_team_name, self.away_team_name, self.time
        )
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

pub fn get_matches(root: Vec<Event>) -> std::string::String {
    let mut match_array: Vec<TennisMatch> = Vec::new();

    for team in root {
        if (team.tournament.category.flag == "atp") && (team.status.type_field == "notstarted") {
            let time = Utc
                .timestamp_millis_opt(team.start_timestamp.unwrap())
                .unwrap();
            let time_minute = time.minute().to_string();
            let (is_pm, time) = time.hour12();
            let time_updated = if is_pm { "PM" } else { "AM" };
            let match_time = time.to_string() + ":" + &time_minute + time_updated + " EST";
            let match_builder: TennisMatch = TennisMatch {
                home_team_name: team.home_team.name,
                away_team_name: team.away_team.name,
                time: match_time,
            };
            match_array.push(match_builder)
        };
    }
    let fmt_match_array: String = match_array
        .iter()
        .format_with("\n", |tennis, f| f(&format_args!("{}", tennis)))
        .to_string();
    return fmt_match_array;
}

pub async fn send_live(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

    let url: String = format!("{}?rapidapi-key={}", LIVE_URL, api_key);

    let request: Request = client.get(url).build().unwrap();

    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_matches(resp.events);

    Ok(match_results)
}

pub async fn send_today_schedule(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let dt: DelayedFormat<StrftimeItems> = Local::now().format("%d/%m/%Y");
    const SCHED_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/";
    let url: String = format!("{}{}?rapidapi-key={}", SCHED_URL, dt, api_key);
    println!("{}", url);
    let request: Request = client.get(url).build().unwrap();
    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let match_results: String = get_matches(resp.events);

    Ok(match_results)
}
