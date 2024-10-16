
#[derive(Clone)]
pub struct Teams {
    pub home : String,
    pub away : String,
}

pub struct Fixture {
    teams       : Teams,
    date        : String,
    time        : String,
    competition : String,
}

pub struct FixtureList {
    fixtures    : Vec<Fixture>,
}

impl Teams {

    pub fn new() -> Teams {
        Teams { home: String::from("TEAM PLACEHOLDER"), away: String::from("TEAM PLACEHOLDER") }
    }
}

impl Fixture {
    pub fn new(teams : Teams, date : String, time : String, competition : String) -> Fixture {
        Fixture { teams: teams, date: date, time: time, competition: competition }
    }

    pub fn print_fixture(&self) {
        println!("{:?} vs {:?}", self.teams.home, self.teams.away);
        println!("{:?}", self.competition);
        println!("{:?}, {:?}\n", self.date, self.time);
    }
}

impl FixtureList {
    pub fn new () -> FixtureList {
        FixtureList{ fixtures: Vec::<Fixture>::new() }
    }

    pub fn push_fx(&mut self, fx : Fixture) {
        self.fixtures.push(fx);
    }

    pub fn get_len(&self) -> usize {
        return self.fixtures.len();
    }

    pub fn print_flist(&self, n : usize) {
        for i in 0..n {
            self.fixtures[i].print_fixture();
        }
    }
}

