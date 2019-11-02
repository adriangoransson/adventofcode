//
use std::collections::HashMap;
use std::io::{stdin, Read};
use std::str::FromStr;

#[derive(PartialEq)]
enum GuardAction {
    GuardShift { id: u32 },
    FallAsleep,
    WakeUp,
}
struct LogEntry {
    minute: usize,
    action: GuardAction,
}

impl FromStr for LogEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let minute = s[15..17].parse().unwrap();

        let action = match &s[19..24] {
            "Guard" => {
                let guard_id = &s[24..]
                    .trim()
                    .trim_matches('#')
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                GuardAction::GuardShift { id: *guard_id }
            }
            "falls" => GuardAction::FallAsleep,
            "wakes" => GuardAction::WakeUp,
            seq => unreachable!("Unexpected sequence: {}", seq),
        };

        Ok(LogEntry { minute, action })
    }
}

fn add_minutes_to_sleep_schedule(logs: &[LogEntry], guard: &mut [u32; 60]) {
    logs.iter().fold(0, |start, log| {
        if log.action == GuardAction::WakeUp {
            (start..log.minute).for_each(|m| guard[m] += 1);
        }

        log.minute
    });
}

fn run(input: &str) -> (u32, u32) {
    let mut data: Vec<&str> = input.lines().collect();
    data.sort();
    let mut lines = data.iter();

    let mut guards: HashMap<u32, [u32; 60]> = HashMap::new();
    let guard_default = || [0; 60];

    let guard_id = match lines.next().unwrap().parse::<LogEntry>().unwrap().action {
        GuardAction::GuardShift { id } => id,
        _ => unreachable!("First line should be a guard shift"),
    };

    let mut guard = guards.entry(guard_id).or_insert_with(guard_default);
    let mut logs: Vec<LogEntry> = Vec::new();

    for line in lines {
        let log = line.parse::<LogEntry>().expect("Failed to parse log entry");

        match log.action {
            GuardAction::GuardShift { id } => {
                add_minutes_to_sleep_schedule(&logs, &mut guard);
                logs.clear();

                guard = guards.entry(id).or_insert_with(guard_default);
            }
            _ => logs.push(log),
        }
    }

    add_minutes_to_sleep_schedule(&logs, &mut guard);

    let (guard, schedule) = guards
        .iter()
        .max_by_key(|(_, schedule)| {
            let sum: u32 = schedule.iter().sum();
            sum
        })
        .unwrap();

    let (sleepiest_minute, _) = schedule.iter().enumerate().max_by_key(|x| x.1).unwrap();

    let part1 = *guard * sleepiest_minute as u32;

    let (guard, (minute, _)) = guards
        .iter()
        .map(|(guard, schedule)| {
            (
                guard,
                schedule.iter().enumerate().max_by_key(|x| x.1).unwrap(),
            )
        })
        .max_by_key(|(_, x)| x.1)
        .unwrap();

    let part2 = *guard * minute as u32;

    (part1, part2)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (part1, part2) = run(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"
        .into()
    }

    #[test]
    fn part1() {
        let (part1, _) = run(&data());
        assert_eq!(240, part1);
    }

    #[test]
    fn part2() {
        let (_, part2) = run(&data());
        assert_eq!(4455, part2);
    }
}
