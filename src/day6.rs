#[derive(Debug)]
pub struct Races {
    times: Vec<u64>,
    distances: Vec<u64>
}

impl Races {
    pub fn parse(input: &str) -> Option<Races> {
        let mut lines = input.lines();
        let time_line = lines.next()?.strip_prefix("Time:")?;
        let distance_line = lines.next()?.strip_prefix("Distance:")?;

        let times = time_line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
        let distances = distance_line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();

        Some(Races {
            times,
            distances
        })
    }

    pub fn one_race(&self) -> Race {
        let time: String = self.times.iter().map(|n| n.to_string()).collect();
        let distance: String = self.distances.iter().map(|n| n.to_string()).collect();

        Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        }
    }

    pub fn races(&self) -> Vec<Race> {
        self.times.iter()
            .zip(&self.distances)
            .map(|(time, distance)| Race::new(*time, *distance))
            .collect()
    }
}

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Race {
        Race {
            time,
            distance
        }
    }

    pub fn possible_winning_scores(&self) -> impl Iterator<Item = Attempt> + '_ {
        (0..self.time)
            .map(|n| Attempt::new_naive(n, self.time))
            .filter(|a| self.attempt_is_winning(a))
    }

    pub fn attempt_is_winning(&self, attempt: &Attempt) -> bool {
        attempt.distance > self.distance
    }
}

pub struct Attempt {
    distance: u64
}

impl Attempt {
    pub fn new_naive(time_accelerating: u64, max_time: u64) -> Attempt {
        let speed = time_accelerating;
        let race_time = max_time - time_accelerating;
        let distance = speed * race_time;

        Attempt {
            distance
        }
    }
}