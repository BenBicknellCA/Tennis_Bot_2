use itertools::Itertools;
use reqwest::Client;
use std::fmt;
extern crate serde;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub away_score: AwayScore,
    pub away_team: Team,
    pub changes: Changes,
    pub custom_id: String,
    pub final_result_only: bool,
    pub first_to_serve: Option<i64>,
    pub has_global_highlights: bool,
    pub home_score: HomeScore,
    pub home_team: Team,
    pub id: Option<i64>,
    pub last_period: String,
    pub periods: Periods,
    pub round_info: RoundInfo,
    pub slug: String,
    pub start_timestamp: Option<i64>,
    pub status: Status,
    pub time: Time,
    pub tournament: Tournament,
    pub winner_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwayScore {
    pub current: Option<i64>,
    pub display: Option<i64>,
    pub normaltime: Option<Value>,
    pub period1: Option<i64>,
    #[serde(rename = "period1TieBreak")]
    pub period1tie_break: Option<i64>,
    pub period2: Option<i64>,
    #[serde(rename = "period2TieBreak")]
    pub period2tie_break: Option<Value>,
    pub period3: Option<i64>,
    #[serde(rename = "period3TieBreak")]
    pub period3tie_break: Option<Value>,
    pub period4: Option<Value>,
    pub period5: Option<Value>,
    #[serde(rename = "period5TieBreak")]
    pub period5tie_break: Option<Value>,
    pub point: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub disabled: Value,
    pub id: Option<i64>,
    pub name: String,
    pub short_name: String,
    pub slug: String,
    pub sport: Sport,
    pub sub_teams: Vec<SubTeam>,
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
    pub disabled: Value,
    pub gender: String,
    pub id: Option<i64>,
    pub name: String,
    pub name_code: String,
    pub national: bool,
    pub ranking: Option<i64>,
    pub short_name: String,
    pub slug: String,
    pub sport: Sport2,
    pub sub_teams: Vec<Value>,
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
    pub changes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeScore {
    pub current: Option<i64>,
    pub display: Option<i64>,
    pub normaltime: Option<Value>,
    pub period1: Option<i64>,
    #[serde(rename = "period1TieBreak")]
    pub period1tie_break: Option<i64>,
    pub period2: Option<i64>,
    #[serde(rename = "period2TieBreak")]
    pub period2tie_break: Option<Value>,
    pub period3: Option<i64>,
    #[serde(rename = "period3TieBreak")]
    pub period3tie_break: Option<Value>,
    pub period4: Option<Value>,
    pub period5: Option<Value>,
    #[serde(rename = "period5TieBreak")]
    pub period5tie_break: Option<Value>,
    pub point: Option<String>,
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
    pub sub_teams: Vec<Value>,
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
    pub has_position_graph: Value,
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
}

impl fmt::Display for TennisMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs. {}", self.home_team_name, self.away_team_name)
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

// impl fmt::Display for Vec<TennisMatch> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.iter().fold(Ok(()), |result, tennis_match| {
//             result.and_then(writeln!(f, "{}", tennis_match))
//         })
//     }
// }

pub fn get_matches(root: Vec<Event>) -> Vec<TennisMatch> {
    let mut match_array: Vec<TennisMatch> = Vec::new();
    for team in root {
        let match_builder = TennisMatch {
            home_team_name: team.home_team.name,
            away_team_name: team.away_team.name,
        };
        match_array.push(match_builder);
    }
    return match_array;
}

pub async fn send_matches(
    api_key: &str,
    client: &Client,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    const LIVE_URL: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

    let url = format!("{}?rapidapi-key={}", LIVE_URL, api_key);

    let request = client.get(url).build().unwrap();

    let resp: Root = client.execute(request).await?.json::<Root>().await?;
    let event = resp.events;
    let match_results = get_matches(event);
    let res = match_results
        .iter()
        .format_with("\n", |tennis, f| f(&format_args!("{}", tennis)))
        .to_string();
    Ok(res)
}
