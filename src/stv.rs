/* good_stv - a good single transferable vote utility.
 * Copyright (C) 2017 Terry Bolt
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

use std::fs::File;
use std::io::Read;
use std::path::Path;

use csv::ReaderBuilder;

use errors::*;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
struct Vote(Vec<String>);

#[derive(Clone, Debug, Default, PartialEq)]
struct Votes {
    pub votes: Vec<Vote>,
}

impl Votes {
    fn new() -> Self {
        Default::default()
    }
}

impl From<Vec<Vote>> for Votes {
    fn from(other: Vec<Vote>) -> Self {
        Votes { votes: other }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Candidate(String);

#[derive(Debug, Default, Deserialize, PartialEq)]
struct Candidates {
    pub candidates: Vec<Candidate>,
}

impl Candidates {
    fn new() -> Self {
        Default::default()
    }
}

impl From<Vec<Candidate>> for Candidates {
    fn from(other: Vec<Candidate>) -> Self {
        Candidates { candidates: other }
    }
}

#[derive(Debug)]
pub struct BallotResults(Vec<(Candidate, u64)>);

#[derive(Debug, Default)]
pub struct Ballot {
    candidates: Candidates,
    votes: Votes,
    seats: u64,
}

impl Ballot {
    pub fn from_csv_file<P: AsRef<Path>>(path: P, seats: u64) -> Result<Self> {
        let file = File::open(path)?;
        Ballot::from_reader(file, seats)
    }

    pub fn from_reader<R: Read>(reader: R, seats: u64) -> Result<Self> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(reader);
        let candidates = csv_reader
            .headers()
            .chain_err(|| "Error parsing CSV header.")?
            .deserialize(None)
            .chain_err(|| "Error deserializing CSV into Candidates struct.")?;

        let mut votes = Vec::new();
        for record in csv_reader.deserialize() {
            let vote: Vote = record.chain_err(|| "Could not deserialize record.")?;
            votes.push(vote);
        }

        Ok(Ballot {
            candidates: candidates,
            votes: votes.into(),
            seats: seats,
        })
    }

    pub fn total_votes(&self) -> u64 {
        self.votes.votes.len() as u64
    }

    pub fn quota(&self) -> u64 {
        (self.total_votes() / (self.seats + 1)) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_csv() {
        let test_csv = "head1,head2,head3\nrecord1,record2";
        let cursor = Cursor::new(test_csv);

        let ballot = Ballot::from_reader(cursor, 10).unwrap();

        assert_eq!(
            ballot.candidates,
            Candidates::from(vec![
                Candidate("head1".to_owned()),
                Candidate("head2".to_owned()),
                Candidate("head3".to_owned()),
            ])
        );
        assert_eq!(
            ballot.votes,
            Votes::from(vec![Vote(vec!["record1".to_owned(), "record2".to_owned()])])
        );
    }

    #[test]
    fn test_quota_calculation() {
        let votes = Votes::from(vec![Vote::default(); 100]);
        let ballot = Ballot {
            votes: votes,
            seats: 2,
            ..Default::default()
        };

        assert_eq!(ballot.quota(), 34);
    }
}
