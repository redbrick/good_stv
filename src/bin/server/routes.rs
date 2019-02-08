/* good_stv - a good single transferable vote utility.
 * Copyright (C) 2019 good_stv authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::iter;

use log::info;
use rand::Rng;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::response::content;
use rocket::response::status::Created;
use rocket::*;
use rocket_contrib::json::Json;
use serde_json::json;

use crate::poll::{Poll, PollCreationRequest};
use crate::PollDb;

const ADMIN_KEY_LENGTH: usize = 8;

#[post("/polls", data = "<poll_req>")]
pub fn create_poll(
    poll_req: Json<PollCreationRequest>,
    poll_db: State<PollDb>,
) -> Created<content::Json<String>> {
    let poll = Poll::new(poll_req.name.clone(), poll_req.candidates.clone());
    info!("Created poll: {:#?}", poll);

    let poll_id = poll.id.clone();
    poll_db.polls.write().unwrap().insert(poll_id.clone(), poll);

    let admin_key: String = iter::repeat(())
        .map(|()| rand::thread_rng().sample(rand::distributions::Alphanumeric))
        .take(ADMIN_KEY_LENGTH)
        .collect();
    let res = json!({
        "id": poll_id.clone(),
        "admin_key": admin_key,
    });
    Created(
        uri!(get_poll: poll_id).to_string(),
        Some(content::Json(res.to_string())),
    )
}

#[get("/polls/<id>")]
pub fn get_poll(id: String, poll_db: State<PollDb>) -> Option<content::Json<String>> {
    poll_db
        .polls
        .read()
        .unwrap()
        .get(&id)
        .map(|poll| content::Json(serde_json::to_string(&poll).unwrap()))
}

#[post("/polls/<id>", data = "<vote>")]
pub fn vote(
    id: String,
    vote: Json<Vec<String>>,
    poll_db: State<PollDb>,
    mut cookies: Cookies,
) -> Result<(), Status> {
    let mut poll_db_lock = poll_db.polls.write().unwrap();
    let poll = poll_db_lock.get_mut(&id).ok_or(Status::NotFound)?;
    let cookie_name = format!("voted_{}", id);
    if cookies.get_private(&cookie_name).is_some() {
        return Err(Status::Forbidden);
    }
    poll.add_vote(vote.into_inner());
    cookies.add_private(Cookie::new(cookie_name, "true"));
    Ok(())
}

#[post("/polls/<id>/results")]
pub fn close_poll(id: String) {
    unimplemented!()
}

#[get("/polls/<id>/results")]
pub fn get_results(id: String) {
    unimplemented!()
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
