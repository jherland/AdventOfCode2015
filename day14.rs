use std::cmp::min;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn period(&self) -> u32 {
        self.fly_time + self.rest_time
    }

    fn travel(&self, seconds: u32) -> u32 {
        ((seconds / self.period()) * self.fly_time + min(self.fly_time, seconds % self.period()))
            * self.speed
    }
}

fn main() {
    let mut reindeer: HashMap<String, Reindeer> = HashMap::new();
    for line in io::stdin().lines().map(Result::unwrap) {
        let words: Vec<&str> = line.split(' ').collect();
        if let [name, "can", "fly", speed, "km/s", "for", fly_time, "seconds,", "but", "then", "must", "rest", "for", rest_time, "seconds."] =
            words[..]
        {
            let speed: u32 = speed.parse().unwrap();
            let fly_time: u32 = fly_time.parse().unwrap();
            let rest_time: u32 = rest_time.parse().unwrap();
            reindeer.insert(
                name.to_owned(),
                Reindeer {
                    speed,
                    fly_time,
                    rest_time,
                },
            );
        } else {
            unreachable!("Parse error: {line:?}");
        }
    }

    println!(
        "Part 1: {}",
        reindeer.values().map(|r| r.travel(2503)).max().unwrap()
    );

    let mut wins = HashMap::new();
    for name in reindeer.keys() {
        wins.insert(name, 0);
    }
    for t in 1..=2503 {
        let results: Vec<_> = reindeer
            .iter()
            .map(|(name, r)| (r.travel(t), name))
            .collect();
        let max_dist = results.iter().max_by_key(|(dist, _)| dist).unwrap().0;
        for (dist, name) in results {
            if dist == max_dist {
                *wins.get_mut(name).unwrap() += 1;
            }
        }
    }
    println!("Part 2: {}", wins.values().max().unwrap());
}
