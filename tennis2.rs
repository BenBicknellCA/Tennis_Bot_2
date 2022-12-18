use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct2 {
    current: i64,
    display: i64,
    normaltime: i64,
    period1: i64,
    period1TieBreak: (),
    period2: i64,
    period2TieBreak: i64,
    period3: (),
    period3TieBreak: (),
    period4: (),
    period5: (),
    period5TieBreak: (),
    point: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct4 {
    id: i64,
    name: String,
    slug: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct5 {
    primary: String,
    secondary: String,
    text: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct3 {
    disabled: (),
    id: i64,
    name: String,
    shortName: String,
    slug: String,
    sport: Struct4,
    subTeams: Vec<_>,
    teamColors: Struct5,
    r#type: i64,
    userCount: i64,
}

impl Display for Struct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.name)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct6 {
    changeTimestamp: i64,
    changes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct7 {
    current: i64,
    display: i64,
    normaltime: i64,
    period1: i64,
    period1TieBreak: (),
    period2: i64,
    period2TieBreak: i64,
    period3: (),
    period3TieBreak: (),
    period4: (),
    period5: (),
    period5TieBreak: (),
    point: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct9 {
    id: i64,
    name: String,
    slug: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct10 {
    primary: String,
    secondary: String,
    text: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct8 {
    disabled: bool,
    id: i64,
    name: String,
    shortName: String,
    slug: String,
    sport: Struct9,
    subTeams: Vec<_>,
    teamColors: Struct10,
    r#type: i64,
    userCount: i64,
}

impl Display for Struct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.name)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct11 {
    current: String,
    period1: String,
    period2: String,
    period3: String,
    period4: String,
    period5: String,
    point: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct12 {
    cupRoundType: i64,
    name: String,
    round: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct13 {
    code: i64,
    description: String,
    r#type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct14 {
    currentPeriodStartTimestamp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct17 {
    id: i64,
    name: String,
    slug: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct16 {
    flag: String,
    id: i64,
    name: String,
    slug: String,
    sport: Struct17,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct20 {
    id: i64,
    name: String,
    slug: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct19 {
    flag: String,
    id: i64,
    name: String,
    slug: String,
    sport: Struct20,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct18 {
    category: Struct19,
    hasEventPlayerStatistics: bool,
    hasPositionGraph: bool,
    id: i64,
    name: String,
    slug: String,
    userCount: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct15 {
    category: Struct16,
    id: i64,
    name: String,
    priority: i64,
    slug: String,
    uniqueTournament: Struct18,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct1 {
    awayScore: Struct2,
    awayTeam: Struct3,
    changes: Struct6,
    customId: String,
    finalResultOnly: bool,
    firstToServe: i64,
    hasGlobalHighlights: bool,
    homeScore: Struct7,
    homeTeam: Struct8,
    id: i64,
    lastPeriod: (),
    periods: Struct11,
    roundInfo: Struct12,
    slug: String,
    startTimestamp: i64,
    status: Struct13,
    time: Struct14,
    tournament: Struct15,
    winnerCode: i64,
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

pub async fn get_matches(
    api_key: &str,
    client: &Client,
) -> Result<(Struc8, Struct3), Box<dyn std::error::Error>> {
    const MATCHES: &str = "https://tennisapi1.p.rapidapi.com/api/tennis/events/live";

let url = format!("{}?apikey={}", MATCHES, api_key);

let request = client.get(url).build.unwrap();

let resp = client
    .execute(request)
    .await?
    .json::<Vec<Location>>()
    .await?;}