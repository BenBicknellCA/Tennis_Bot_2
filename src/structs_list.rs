// https://rapidapi.com/fluis.lacasse/api/tennisapi1

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
