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
use rocket::response::content;
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
) -> response::status::Created<content::Json<String>> {
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
    response::status::Created(
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

#[post("/polls/<id>")]
pub fn vote(id: String) {
    unimplemented!()
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
