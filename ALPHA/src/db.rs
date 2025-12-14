use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use std::error::Error;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[derive(QueryableByName, Debug)]
struct TablePrivilege {
    #[diesel(sql_type = Text)]
    table_schema: String,
    #[diesel(sql_type = Text)]
    table_name: String,
    #[diesel(sql_type = Text)]
    privilege_type: String,
}

pub fn ensure_read_only(
    pool: &Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<(), Box<dyn Error>> {
    let conn = &mut pool.get()?;

    let results: Vec<TablePrivilege> = sql_query(
        "SELECT table_schema, table_name, privilege_type
         FROM information_schema.role_table_grants
         WHERE grantee = CURRENT_USER
           AND privilege_type IN ('INSERT', 'UPDATE', 'DELETE')",
    )
    .load(conn)?;

    if !results.is_empty() {
        eprintln!("🚨 Database user has write privileges on the following tables:");
        for priv_row in results {
            eprintln!(
                "- {}.{} ({} access)",
                priv_row.table_schema, priv_row.table_name, priv_row.privilege_type
            );
        }
        return Err("Database user has write privileges. Aborting.".into());
    }

    println!("✅ Database user is read-only.");
    Ok(())
}
