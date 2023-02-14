use reqwest::Client;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

extern crate serde;

#[derive(Debug, Serialize, Deserialize)]
pub struct Live {
    events: Vec<Event>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "awayScore")]
    away_score: HashMap<String, Option<i64>>,
    #[serde(rename = "awayTeam")]
    away_team: Team,
    changes: Changes,
    #[serde(rename = "customId")]
    custom_id: String,
    #[serde(rename = "finalResultOnly")]
    final_result_only: bool,
    #[serde(rename = "firstToServe")]
    first_to_serve: Option<serde_json::Value>,
    #[serde(rename = "hasGlobalHighlights")]
    has_global_highlights: bool,
    #[serde(rename = "homeScore")]
    home_score: HashMap<String, Option<i64>>,
    #[serde(rename = "homeTeam")]
    home_team: Team,
    id: i64,
    #[serde(rename = "lastPeriod")]
    last_period: String,
    periods: Periods,
    #[serde(rename = "roundInfo")]
    round_info: RoundInfo,
    slug: String,
    #[serde(rename = "startTimestamp")]
    start_timestamp: i64,
    status: Status,
    time: Time,
    tournament: Tournament,
    #[serde(rename = "winnerCode")]
    winner_code: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    disabled: Option<bool>,
    id: i64,
    name: String,
    #[serde(rename = "shortName")]
    short_name: String,
    slug: String,
    sport: Sport,
    #[serde(rename = "subTeams")]
    sub_teams: Vec<Option<serde_json::Value>>,
    #[serde(rename = "teamColors")]
    team_colors: TeamColors,
    #[serde(rename = "type")]
    team_type: i64,
    #[serde(rename = "userCount")]
    user_count: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sport {
    id: i64,
    name: String,
    slug: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamColors {
    primary: String,
    secondary: String,
    text: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {
    #[serde(rename = "changeTimestamp")]
    change_timestamp: i64,
    changes: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Periods {
    current: String,
    period1: String,
    period2: String,
    period3: String,
    period4: String,
    period5: String,
    point: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RoundInfo {
    #[serde(rename = "cupRoundType")]
    cup_round_type: Option<serde_json::Value>,
    name: String,
    round: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    code: i64,
    description: String,
    #[serde(rename = "type")]
    status_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    #[serde(rename = "currentPeriodStartTimestamp")]
    current_period_start_timestamp: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    category: Category,
    id: i64,
    name: String,
    priority: i64,
    slug: String,
    #[serde(rename = "uniqueTournament")]
    unique_tournament: UniqueTournament,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    flag: String,
    id: i64,
    name: String,
    slug: String,
    sport: Sport,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UniqueTournament {
    category: Category,
    #[serde(rename = "hasEventPlayerStatistics")]
    has_event_player_statistics: bool,
    #[serde(rename = "hasPositionGraph")]
    has_position_graph: Option<serde_json::Value>,
    id: i64,
    name: String,
    slug: String,
    #[serde(rename = "userCount")]
    user_count: i64,
}
impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[derive(Debug)]
pub struct CouldNotFindPlayer {
    name: String,
}

impl Display for CouldNotFindPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Placeholder 1 '{}'", self.name)
    }
}

// pub async fn get_matches(
//     api_key: &str,
//     client: &Client,
// ) -> Result<Live, Box<dyn std::error::Error>> {
//     const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

//     let url = format!("{}?rapidapi-key={}", LIVE_URL, api_key);

//     let request = client.get(url).build().unwrap();

//     let resp = client.execute(request).await?.json::<Live>().await?;

//     Ok(resp)
// }

pub async fn get_matches(
    api_key: &str,
    client: &Client,
) -> Result<Live, Box<dyn std::error::Error>> {
    const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

    let url = format!("{}?rapidapi-key={}", LIVE_URL, api_key);

    let request = client.get(url).build().unwrap();

    let resp: Live = client.execute(request).await?.json().await?;

    Ok(resp)
}
