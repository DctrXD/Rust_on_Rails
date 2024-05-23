use diesel_migrations::{Migration, RunMigrationsError};
use diesel_migrations::m20231027100000_create_users::schema; 

#[derive(Iden)]
enum CreateUsers {
    Up = "20231027100000_create_users_up",
    Down = "20231027100000_create_users_down",
}

fn up(conn: &mut impl Connection) -> Result<(), RunMigrationsError> {
    conn.build_table(schema::users::table)
        .with_column(schema::users::id.integer().primary().increments(true))
        .with_column(schema::users::name.varchar(255).not_null())
        .with_column(schema::users::email.varchar(255).not_null())
        .execute()?;
    Ok(())
}

fn down(conn: &mut impl Connection) -> Result<(), RunMigrationsError> {
    conn.drop_table(schema::users::table).cascade().execute()?;
    Ok(())
}