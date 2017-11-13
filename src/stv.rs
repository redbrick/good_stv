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

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use csv::ReaderBuilder;
use rand;

use errors::*;

type Candidate = String;
type CandidateVotesPair = (Candidate, Vec<Vote>);
type CandidateVotesMap = HashMap<Candidate, Vec<Vote>>;
type Vote = Vec<String>;

#[derive(Debug, Default, PartialEq)]
pub struct ElectionResults {
    pub elected: HashMap<Candidate, u64>,
    pub eliminated: HashMap<Candidate, u64>,
}

#[derive(Debug, Default)]
pub struct Election {
    candidates: Vec<Candidate>,
    elected: CandidateVotesMap,
    eliminated: CandidateVotesMap,
    num_spoiled_votes: u64,
    seats: u64,
    votes: Vec<Vote>,
}

impl Election {
    pub fn from_csv_file<P: AsRef<Path>>(path: P, seats: u64) -> Result<Self> {
        let file = File::open(&path).chain_err(|| {
            format!("Error opening file {:?}.", path.as_ref().to_string_lossy())
        })?;
        Election::from_reader(file, seats)
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

        let mut election = Election {
            candidates: candidates,
            seats: seats,
            votes: votes,
            ..Default::default()
        };
        let num_spoiled_votes = election.purge_spoiled_votes();
        info!("{} spoiled votes purged.", num_spoiled_votes);
        election.num_spoiled_votes = num_spoiled_votes;

        Ok(election)
    }

    pub fn total_votes(&self) -> u64 {
        self.votes.len() as u64
    }

    pub fn quota(&self) -> u64 {
        (self.total_votes() / (self.seats + 1)) + 1
    }

    pub fn results(mut self) -> Result<ElectionResults> {
        let mut candidate_votes = CandidateVotesMap::new();
        for candidate in &self.candidates {
            candidate_votes.insert(candidate.clone(), Vec::new());
        }

        // First-choice votes
        for vote in &self.votes {
            let candidate = candidate_votes.get_mut(&vote[0]).unwrap();
            candidate.push(vote.clone());
        }

        while self.elected.len() < self.seats as usize {
            let elected_this_round = self.get_round_winners(&candidate_votes);
            self.elected.extend(elected_this_round.clone().into_iter());
            // If there were winners this round, redistribute their surplus votes and remove them
            // from candidate_votes.
            if !elected_this_round.is_empty() {
                for (candidate, votes) in &elected_this_round {
                    let num_surplus = self.distribute_winner_excess(
                        &(candidate.clone(), votes.clone()),
                        &mut candidate_votes,
                    );
                    candidate_votes.remove(candidate);
                    info!("{:?} redistributed from winner surplus", num_surplus);
                }
            } else {
                // If there were no winners this round, choose a loser, eliminate them, and
                // distribute their votes.
                let loser = self.get_round_loser(&candidate_votes)?;
                self.eliminated.insert(loser.0.clone(), loser.1.clone());
                let num_redistributed_votes =
                    self.distribute_loser_votes(&loser, &mut candidate_votes);
                candidate_votes.remove(&loser.0);
                info!("{:?} redistributed from loser", num_redistributed_votes);
            }
        }

        Ok(ElectionResults {
            elected: self.elected
                .into_iter()
                .map(|(k, v): (Candidate, Vec<Vote>)| (k, v.len() as u64))
                .collect(),
            eliminated: self.eliminated
                .into_iter()
                .map(|(k, v): (Candidate, Vec<Vote>)| (k, v.len() as u64))
                .collect(),
        })
    }

    // A spoiled vote is a vote containing a candidate who doesn't exist.
    fn purge_spoiled_votes(&mut self) -> u64 {
        let before_length = self.votes.len();
        let candidates = self.candidates.as_slice();
        self.votes.retain(|vote| {
            for candidate in vote {
                if !candidates.contains(candidate) {
                    info!("Candidate voted for but not running: {}.", candidate);
                    return false;
                }
            }
            true
        });
        (before_length - self.votes.len()) as u64
    }

    fn get_round_winners(
        &self,
        candidate_votes: &CandidateVotesMap,
    ) -> HashMap<Candidate, Vec<Vote>> {
        let mut elected = HashMap::new();
        for (candidate, votes) in candidate_votes {
            if votes.len() >= self.quota() as usize {
                elected.insert(candidate.clone(), votes.clone());
            }
        }
        elected
    }

    fn get_round_loser(&self, candidate_votes: &CandidateVotesMap) -> Result<CandidateVotesPair> {
        let loser = candidate_votes.iter().min_by(
            |a, b| a.1.len().cmp(&b.1.len()),
        );
        loser.map(|(k, v)| (k.clone(), v.clone())).ok_or_else(|| {
            "Could not choose a loser.".into()
        })
    }

    fn distribute_winner_excess(
        &self,
        candidate: &CandidateVotesPair,
        candidate_votes: &mut CandidateVotesMap,
    ) -> u64 {
        // Calculate how many surplus votes to distribute.
        let num_surplus = candidate.1.len() - self.quota() as usize;
        let surplus_votes = rand::sample(
            &mut rand::thread_rng(),
            candidate.1.clone().into_iter(),
            num_surplus,
        );

        for vote in &surplus_votes {
            if vote.len() == 1 {
                continue;
            }
            let new_vote = &vote[1..];
            // Don't assign votes to people already elected or elmininated.
            if self.elected.contains_key(&new_vote[0]) ||
                self.eliminated.contains_key(&new_vote[0])
            {
                continue;
            }
            let cand = candidate_votes.get_mut(&new_vote[0]).unwrap();
            cand.push(new_vote.to_vec());
        }

        num_surplus as u64
    }

    fn distribute_loser_votes(
        &self,
        candidate: &CandidateVotesPair,
        candidate_votes: &mut CandidateVotesMap,
    ) -> u64 {
        for vote in &candidate.1 {
            if vote.len() == 1 {
                continue;
            }
            let new_vote = &vote[1..];
            // Don't assign votes to people already elected or elmininated.
            if self.elected.contains_key(&new_vote[0]) ||
                self.eliminated.contains_key(&new_vote[0])
            {
                continue;
            }
            let cand = candidate_votes.get_mut(&new_vote[0]).unwrap();
            cand.push(new_vote.to_vec());
        }
        candidate.1.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_csv() {
        let test_csv = "cand1,cand2,cand3\ncand1,cand2";
        let cursor = Cursor::new(test_csv);

        let election = Election::from_reader(cursor, 10).unwrap();

        assert_eq!(
            election.candidates,
            vec!["cand1".to_owned(), "cand2".to_owned(), "cand3".to_owned()]
        );
        assert_eq!(
            election.votes,
            vec![vec!["cand1".to_owned(), "cand2".to_owned()]]
        );
    }

    #[test]
    fn test_quota_calculation() {
        let votes = vec![Vote::default(); 100];
        let election = Election {
            votes: votes,
            seats: 2,
            ..Default::default()
        };

        assert_eq!(election.quota(), 34);
    }

    #[test]
    fn test_election_results() {
        let expected_results = ElectionResults {
            elected: {
                let mut elected = HashMap::new();
                elected.insert("a".to_owned(), 4);
                elected.insert("c".to_owned(), 4);
                elected
            },
            eliminated: {
                let mut eliminated = HashMap::new();
                eliminated.insert("b".to_owned(), 2);
                eliminated.insert("d".to_owned(), 1);
                eliminated
            },
        };
        let test_csv = "a,b,c,d\nc,b,a\nc,b,a\nb,c\na,b\nc,b\nb,a\nc,b,a\nd,a\na,b";
        let cursor = Cursor::new(test_csv);
        let election = Election::from_reader(cursor, 2).unwrap();

        let results = election.results().unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_spoiled_vote_removal() {
        let expected_results = ElectionResults {
            elected: {
                let mut elected = HashMap::new();
                elected.insert("a".to_owned(), 3);
                elected
            },
            ..Default::default()
        };
        let test_csv = "a\na\na\nz\na";
        let cursor = Cursor::new(test_csv);
        let election = Election::from_reader(cursor, 1).unwrap();
        assert_eq!(1, election.num_spoiled_votes);

        let results = election.results().unwrap();
        assert_eq!(expected_results, results);
    }
}
