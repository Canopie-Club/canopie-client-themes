use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use std::error::Error;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
