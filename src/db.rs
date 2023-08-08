use crate::models::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;

type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_all_projects(conn: &mut DbConnection) -> QueryResult<Vec<Project>> {
    use crate::schema::project::dsl::*;
    project.load::<Project>(conn)
}

pub fn get_all_skills(conn: &mut DbConnection) -> QueryResult<Vec<Skill>> {
    use crate::schema::skill::dsl::*;

    skill.load::<Skill>(conn)
}

pub fn get_all_bio(conn: &mut DbConnection) -> QueryResult<Bio> {
    use crate::schema::bio::dsl::*;

    let last_bio = bio
        .order(id.desc()) // Order by `id` in descending order (last row first)
        .limit(1) // Limit the result to one row
        .load::<Bio>(conn)?
        .pop(); // Get the last row as an Option<Bio>

    Ok(last_bio.unwrap())
}
