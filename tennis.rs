use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Struct1 {
    events: Vec<Events>,
}

impl Display for Struct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.AwayTeam, self.HomeTeam)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Events {
    pub awayScore: AwayScore,
    pub awayTeam: AwayTeam,
    pub changes: Changes,
    pub customId: String,
    pub finalResultOnly: bool,
    pub firstToServe: i64,
    pub hasGlobalHighlights: bool,
    pub homeScore: HomeScore,
    pub homeTeam: HomeTeam,
    pub lastPeriod: (),
    pub periods: Periods,
    pub roundInfo: RoundInfo,
    pub slug: String,
    pub startTimestamp: i64,
    pub status: Status,
    pub time: Time,
    pub tournament: Tournament,
    pub winnerCode: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct AwayScore {
    pub current: i64,
    pub display: i64,
    pub normaltime: i64,
    pub period1: i64,
    pub period1TieBreak: (),
    pub period2: i64,
    pub period2TieBreak: i64,
    pub period3: (),
    pub period3TieBreak: (),
    pub period4: (),
    pub period5: (),
    pub period5TieBreak: (),
    pub point: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct AwayTeam {
    pub disabled: (),
    pub id: i64,
    pub name: String,
    pub shortName: String,
    pub slug: String,
    pub sport: SportAway,
    pub subTeams: Vec<_>,
    pub teamColors: TeamColors,
    pub r#type: i64,
    pub serCount: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct SportCat {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Changes {
    pub changeTimestamp: i64,
    pub changes: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct HomeScore {
    pub current: i64,
    pub display: i64,
    pub period1: i64,
    pub period1TieBreak: (),
    pub period2: i64,
    pub period2TieBreak: i64,
    pub period3: (),
    pub period3TieBreak: (),
    pub period4: (),
    pub period5: (),
    pub period5TieBreak: (),
    pub point: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct HomeTeam {
    pub disabled: bool,
    pub id: i64,
    pub name: String,
    pub shortName: String,
    pub slug: String,
    pub sport: SportHome,
    pub subTeams: Vec<_>,
    pub teamColors: TeamColorsHome,
    pub r#type: i64,
    pub userCount: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct TeamColorsHome {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SportHome {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct RoundInfo {
    pub cupRoundType: i64,
    pub name: String,
    pub round: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Status {
    pub code: i64,
    pub description: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Time {
    pub currentPeriodStartTimestamp: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Tournament {
    pub category: Category,
    pub id: i64,
    pub name: String,
    pub priority: i64,
    pub slug: String,
    pub uniqueTournament: UniqueTournament,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct UniqueTournament {
    pub category: CategoryUnique,
    pub hasEventPlayerStatistics: bool,
    pub hasPositionGraph: bool,
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub userCount: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Category {
    pub flag: String,
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub sport: SportCat,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct CategoryUnique {
    pub flag: String,
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub sport: SportCatUni,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct SportCatUni {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct SportAway {
    pub id: i64,
    pub name: String,
    pub slug: String,
}
