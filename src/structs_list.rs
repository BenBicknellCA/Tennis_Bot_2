// https://rapidapi.com/fluis.lacasse/api/tennisapi1
use chrono::{Local, NaiveDateTime};
use chrono_tz::America::Toronto;
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
    pub home_team_seed: Option<String>,
    pub away_team_seed: Option<String>,
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

impl Event {
    pub fn get_time(&self) -> chrono::DateTime<chrono_tz::Tz> {
        let binding = &self.time;
        let accurate_start = self.start_timestamp;
        let time = binding.as_ref().unwrap();
        let current_period = time.current_period_start_timestamp;
        let time_to_return = if let Some(accurate_start) = accurate_start {
            NaiveDateTime::from_timestamp_opt(accurate_start, 0).expect("Match missing time4")
        } else {
            NaiveDateTime::from_timestamp_opt(
                current_period.expect("current period start timestamp"),
                0,
            )
            .expect("Match missing time6")
        };
        time_to_return
            .and_local_timezone(chrono::Utc)
            .unwrap()
            .with_timezone(&Toronto)
    }
    pub fn hr_min_ampm_tz(&self) -> String {
        self.get_time().format("%l:%M %p %Z").to_string()
    }
    pub fn day_mnth_yr(&self) -> String {
        self.get_time().format("%d/%m/%Y").to_string()
    }

    pub fn live_time(&self) -> String {
            if self.status.type_field == "inprogress" {"".to_string()} else {format!("/ {}", self.hr_min_ampm_tz())}
    }

    //    pub fn fmt_match(&self) -> Event {
    //        Event {
    //        home_team: self.home_team.name,
    //        away_team_name: self.away_team.name,
    //        time: self.hr_min_ampm_tz()
    //        }
    //    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    // pub disabled: Option<Value>,
    pub country: Country,
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
    pub country: Country,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub alpha2: Option<String>,
    pub name: Option<String>,
}

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
        write!(
            f,
            "{}  :flag_{}: ",
            self.name,
            self.country
                .alpha2
                .as_ref()
                .unwrap()
                .to_lowercase()
                .as_str()
        )
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs. {} {}",
            self.home_team,
            self.away_team,
            self.live_time()
        )
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

// api format

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Score {
//     pub current: Option<i64>,
//     pub display: Option<i64>,
//     pub normaltime: Option<Value>,
//     pub period1: Option<i64>,
//     #[serde(rename = "period1TieBreak")]
//     pub period1tie_break: Option<i64>,
//     pub period2: Option<i64>,
//     #[serde(rename = "period2TieBreak")]
//     pub period2tie_break: Option<i64>,
//     pub period3: Option<i64>,
//     #[serde(rename = "period3TieBreak")]
//     pub period3tie_break: Option<i64>,
//     pub period4: Option<i64>,
//     pub period5: Option<i64>,
//     #[serde(rename = "period5TieBreak")]
//     pub period5tie_break: Option<i64>,
//     pub point: Option<String>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport {
//     pub id: Option<i64>,
//     pub name: String,
//     pub slug: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SubTeam {
//     pub disabled: Option<Value>,
//     pub gender: Option<String>,
//     pub id: Option<i64>,
//     pub name: String,
//     pub name_code: Option<String>,
//     pub national: Option<Value>,
//     pub ranking: Option<i64>,
//     pub short_name: String,
//     pub slug: String,
//     pub sport: Sport2,
//     pub team_colors: TeamColors,
//     #[serde(rename = "type")]
//     pub type_field: Option<i64>,
//     pub user_count: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport2 {
//     pub id: Option<i64>,
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

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TeamColors2 {
//     pub primary: String,
//     pub secondary: String,
//     pub text: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Changes {
//     pub change_timestamp: Option<i64>,
//     pub changes: Option<Vec<String>>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport3 {
//     pub id: Option<i64>,
//     pub name: String,
//     pub slug: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SubTeam2 {
//     pub disabled: Value,
//     pub gender: String,
//     pub id: Option<i64>,
//     pub name: String,
//     pub name_code: String,
//     pub national: bool,
//     pub ranking: Option<i64>,
//     pub short_name: String,
//     pub slug: String,
//     pub sport: Sport4,
//     pub sub_teams: Option<Vec<Value>>,
//     pub team_colors: TeamColors3,
//     #[serde(rename = "type")]
//     pub type_field: Option<i64>,
//     pub user_count: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport4 {
//     pub id: Option<i64>,
//     pub name: String,
//     pub slug: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TeamColors3 {
//     pub primary: String,
//     pub secondary: String,
//     pub text: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TeamColors4 {
//     pub primary: String,
//     pub secondary: String,
//     pub text: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Periods {
//     pub current: String,
//     pub period1: String,
//     pub period2: String,
//     pub period3: String,
//     pub period4: String,
//     pub period5: String,
//     pub point: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RoundInfo {
//     pub cup_round_type: Option<i64>,
//     pub name: String,
//     pub round: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport5 {
//     pub id: Option<i64>,
//     pub name: String,
//     pub slug: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Category2 {
//     pub flag: String,
//     pub id: Option<i64>,
//     pub name: String,
//     pub slug: String,
//     pub sport: Sport6,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sport6 {
//     pub id: Option<i64>,
//     pub name: String,
//     pub slug: String,
//     pub has_position_graph: Option<String>,
// }
