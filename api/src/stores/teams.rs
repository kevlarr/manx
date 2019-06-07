use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::ApiResult;
use crate::models::{Team, NewTeam, Member};
use crate::schema;

pub fn create(conn: &PgConnection, new_team: &NewTeam) -> ApiResult<Team> {
    Ok(diesel::insert_into(schema::teams::table)
        .values(new_team)
        .get_result(conn)?)
}

pub fn for_members(conn: &PgConnection, members: &Vec<Member>) -> ApiResult<Vec<Team>> {
    use schema::teams::dsl::{teams, id};

    let team_ids = members.iter().map(|m| m.team_id);

    Ok(teams.filter(id.eq_any(team_ids)).load(conn)?)
}
