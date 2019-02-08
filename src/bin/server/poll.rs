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

use chrono::{DateTime, Local};
use rand::Rng;
use serde_derive::*;

const ID_LENGTH: usize = 6;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Candidate {
    id: String,
    name: String,
}

impl Candidate {
    pub fn new(name: String) -> Self {
        Candidate {
            id: name.to_lowercase().replace(" ", "_"),
            name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PollCreationRequest {
    pub name: String,
    pub candidates: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Poll {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub in_progress: bool,
    pub candidates: Vec<Candidate>,
}

impl Poll {
    pub fn new(name: String, candidates: Vec<String>) -> Self {
        Poll {
            id: Poll::generate_id(),
            name,
            created_at: Local::now(),
            in_progress: true,
            candidates: candidates
                .iter()
                .map(|candidate| Candidate::new(candidate.to_string()))
                .collect(),
        }
    }

    fn generate_id() -> String {
        iter::repeat(())
            .map(|()| rand::thread_rng().sample(rand::distributions::Alphanumeric))
            .take(ID_LENGTH)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_poll() {
        let expected_candidates = vec![
            Candidate {
                id: "alice".to_string(),
                name: "Alice".to_string(),
            },
            Candidate {
                id: "bob_smith".to_string(),
                name: "Bob Smith".to_string(),
            },
            Candidate {
                id: "charlie".to_string(),
                name: "Charlie".to_string(),
            },
        ];
        let poll = Poll::new(
            "test".to_string(),
            vec![
                "Alice".to_string(),
                "Bob Smith".to_string(),
                "Charlie".to_string(),
            ],
        );
        assert_eq!("test", poll.name);
        assert_eq!(true, poll.in_progress);
        assert_eq!(expected_candidates, poll.candidates);
    }
}
