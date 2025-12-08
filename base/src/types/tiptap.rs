use diesel::expression::AsExpression;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
pub struct TipTapNode {
    #[serde(rename = "type")]
    pub node_type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Value>, // could be a map, number, etc.

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TipTapNode>>, // recursion!

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marks: Option<Value>, // if you need formatting (bold, italic, etc.)
}

use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Jsonb;
use std::io::Write;

impl ToSql<Jsonb, Pg> for TipTapNode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        // Serialize Node -> JSON string
        let json_str = serde_json::to_string(self)
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)?;
        out.write_all(json_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Jsonb, Pg> for TipTapNode {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        let node = serde_json::from_value(value)?;
        Ok(node)
    }
}
