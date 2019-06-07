use actix_web::Json;

use crate::{ApiResult, Request, Response};
use crate::extractors::Credentials;
use crate::models::{
    Team, NewTeam,
    Member, NewMember,
    Stream, NewStream,
    StreamPermission, NewStreamPermission,
    Topic, Comment,
};
use crate::stores;

#[derive(Deserialize)]
pub struct CreateParams {
    title: String,
    username: String,
}

#[derive(Serialize)]
pub struct Created {
    team: Team,
    member: Member,
    stream: Stream,

    #[serde(rename = "streamPermission")]
    stream_permission: StreamPermission,
}

#[derive(Serialize)]
pub struct Listed {
    comments: Vec<Comment>,
    members: Vec<Member>,
    streams: Vec<Stream>,
    teams: Vec<Team>,
    topics: Vec<Topic>,
}

pub fn create((req, cred, params): (Request, Credentials, Json<CreateParams>)) -> ApiResult<Json<Created>> {
    let conn = &req.state().conn;

    // TODO: Transactions? Changesets?
    let new_team = NewTeam::new(cred.user.id, params.title.clone());
    let team = stores::teams::create(conn, &new_team)?;

    let new_member = NewMember::new(cred.user.id, team.id, params.username.clone());
    let member = stores::members::create(conn, &new_member)?;

    let new_stream = NewStream::new(team.id, member.id, team.title.clone());
    let stream = stores::streams::create(conn, &new_stream)?;

    let new_permission = NewStreamPermission::new(member.id, stream.id);
    let stream_permission = stores::stream_permissions::create(conn, &new_permission)?;

    Ok(Json(Created { team, member, stream, stream_permission }))
}

pub fn list((req, cred): (Request, Credentials)) -> ApiResult<Json<Listed>> {
    let conn = &req.state().conn;

    let members  = stores::members::for_user(conn, &cred.user)?;
    let teams    = stores::teams::for_members(conn, &members)?;
    let streams  = stores::streams::for_members(conn, &members)?;
    let topics   = stores::topics::for_streams(conn, &streams)?;
    let comments = stores::comments::for_topics(conn, &topics)?;

    Ok(Json(Listed { comments, members, streams, teams, topics }))
}

pub fn delete(_req: Request) -> Response {
    Response::Ok().finish()
}
