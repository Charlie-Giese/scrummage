// Implementations of structs for storing fixture list information
// Author: Charles Giese

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Teams {
    pub home: String,
    pub away: String,
}

#[derive(Clone)]
pub struct Fixture {
    teams: Teams,
    datetime: DateTime<Local>,
    competition: String,
}

pub struct FixtureList {
    fixtures: Vec<Fixture>,
    idx: usize,
}

impl Teams {
    pub fn new() -> Teams {
        Teams {
            home: String::from("TEAM PLACEHOLDER"),
            away: String::from("TEAM PLACEHOLDER"),
        }
    }
}

impl Fixture {
    pub fn new(teams: Teams, datetime: DateTime<Local>, competition: String) -> Fixture {
        Fixture {
            teams: teams,
            datetime: datetime,
            competition: competition,
        }
    }

    pub fn print_fixture(&self) {
        println!("{:?} vs {:?}", self.teams.home, self.teams.away);
        println!("{:?}", self.competition);
        println!(
            "{:?}\n",
            format!("{}", self.datetime.format("%d-%m-%Y %H:%M"))
        );
    }

    pub fn format_fixture(&self) -> String {
        let fl = format!(
            "{home} vs {away}, {datetime}",
            home = self.teams.home,
            away = self.teams.away,
            datetime = self.datetime.format("%d-%m-%Y %H:%M")
        );

        return fl;
    }
}

impl FixtureList {
    pub fn new() -> FixtureList {
        FixtureList {
            fixtures: Vec::<Fixture>::new(),
            idx: 0,
        }
    }

    pub fn push_fx(&mut self, fx: Fixture) {
        self.fixtures.push(fx);
    }

    pub fn get_len(&self) -> usize {
        return self.fixtures.len();
    }

    pub fn print_flist(&self, n: usize) {
        for i in 0..n {
            self.fixtures[i].print_fixture();
        }
    }
}

impl Iterator for FixtureList {
    type Item = Fixture;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.fixtures.len() {
            let result = self.fixtures[self.idx].clone();
            self.idx += 1;
            Some(result)
        } else {
            None
        }
    }
}
