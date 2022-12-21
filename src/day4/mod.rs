use regex::Regex;

use std::collections::HashMap;
use std::str::FromStr;

type TimeInstant = (u32, u32, u32, u32);

type Event = (TimeInstant, GuardEvent);

#[derive(Eq, PartialEq, Debug)]
pub enum GuardEvent {
    WakeUp,
    FallAsleep,
    BeginShift(u32),
}

pub fn solve1(lines: Vec<Event>) -> u32 {
    let calendar: HashMap<u32, HashMap<u32, u32>> = populate_calendar(lines);

    let (&sleepiest_guard, sleepiest_minutes) = calendar
        .iter()
        .max_by_key(|&(_, v)| v.values().sum::<u32>())
        .unwrap();

    let (&index, _sleeps) = sleepiest_minutes
        .iter()
        .max_by_key(|&(_, no_of_sleeps_of_minute)| no_of_sleeps_of_minute)
        .unwrap();

    sleepiest_guard * index
}

fn populate_calendar(mut lines: Vec<Event>) -> HashMap<u32, HashMap<u32, u32>> {
    let mut calendar: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    sort_lines(&mut lines);

    let mut current_id: Option<u32> = None;
    let mut last_minute = 0u32;

    for &((_, _, _, min), ref event) in lines.iter() {
        match event {
            GuardEvent::BeginShift(id) => {
                current_id = Some(*id);
            }
            GuardEvent::FallAsleep => {
                last_minute = min;
            }
            GuardEvent::WakeUp => {
                let vec = calendar
                    .entry(current_id.unwrap())
                    .or_insert_with(HashMap::new);

                for m in last_minute..min {
                    *vec.entry(m).or_insert(0) += 1;
                }
            }
        }
    }

    calendar
}

fn sort_lines(lines: &mut [Event]) {
    lines.sort_by(|(t1, _), (t2, _)| t1.cmp(t2))
}

fn parse_line(line: &str) -> Event {
    (parse_time(line), parse_event(line))
}

fn parse_time(line: &str) -> TimeInstant {
    let time = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\]").unwrap();
    let caps = time.captures(line).unwrap();
    let caps: Vec<_> = caps
        .iter()
        .skip(1)
        .map(|m| u32::from_str(m.unwrap().as_str()).unwrap())
        .collect();

    (caps[1], caps[2], caps[3], caps[4])
}

fn parse_event(line: &str) -> GuardEvent {
    if line.contains("wakes up") {
        GuardEvent::WakeUp
    } else if line.contains("falls asleep") {
        GuardEvent::FallAsleep
    } else {
        let guard_id = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        let str_id = guard_id.captures(line).unwrap().get(1).unwrap();
        let id = u32::from_str(str_id.as_str()).unwrap();

        GuardEvent::BeginShift(id)
    }
}

pub fn solve2(lines: Vec<Event>) -> u32 {
    let calendar: HashMap<u32, HashMap<u32, u32>> = populate_calendar(lines);

    let (&sleepiest_guard, sleepiest_minutes) = calendar
        .iter()
        .max_by_key(|&(_, v)| v.values().max().unwrap())
        .unwrap();

    let (&index, _sleeps) = sleepiest_minutes
        .iter()
        .max_by_key(|&(_, no_of_sleeps_of_minute)| no_of_sleeps_of_minute)
        .unwrap();

    sleepiest_guard * index
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    #[test]
    fn it_parses_wake_up() {
        let result = parse_event("[1518-11-01 00:25] wakes up");

        assert_eq!(result, GuardEvent::WakeUp)
    }

    #[test]
    fn it_parses_falls_asleep() {
        let result = parse_event("[1518-11-01 00:30] falls asleep");

        assert_eq!(result, GuardEvent::FallAsleep)
    }

    #[test]
    fn it_parses_begins_shift() {
        let result = parse_event("[1518-11-01 00:00] Guard #10 begins shift");
        assert_eq!(result, GuardEvent::BeginShift(10))
    }

    #[test]
    fn it_parses_time() {
        let result = parse_time("[1518-11-01 00:00] Guard #10 begins shift");
        assert_eq!(result, (11, 1, 0, 0))
    }

    #[test]
    fn it_parses_events() {
        let result = parse_line("[1518-09-30 12:34] Guard #77 begins shift");
        assert_eq!(result, ((9, 30, 12, 34), GuardEvent::BeginShift(77)))
    }

    #[test]
    fn it_sorts_events() {
        let mut list = vec![
            ((11, 4, 0, 36), GuardEvent::FallAsleep),
            ((11, 1, 0, 30), GuardEvent::FallAsleep),
            ((11, 1, 0, 55), GuardEvent::WakeUp),
            ((11, 2, 0, 40), GuardEvent::FallAsleep),
            ((11, 1, 0, 25), GuardEvent::WakeUp),
            ((11, 3, 0, 29), GuardEvent::WakeUp),
            ((11, 2, 0, 50), GuardEvent::WakeUp),
            ((11, 4, 0,  2), GuardEvent::BeginShift(99)),
            ((11, 5, 0, 55), GuardEvent::WakeUp),
            ((11, 5, 0, 3), GuardEvent::BeginShift(99)),
            ((11, 1, 0, 0), GuardEvent::BeginShift(10)),
            ((11, 5, 0, 45), GuardEvent::FallAsleep),
            ((11, 4, 0, 46), GuardEvent::WakeUp),
            ((11, 1, 0, 5), GuardEvent::FallAsleep),
            ((11, 1, 23, 58), GuardEvent::BeginShift(99)),
            ((11, 3, 0, 24), GuardEvent::FallAsleep),
            ((11, 3, 0, 5), GuardEvent::BeginShift(10)),
        ];

        sort_lines(&mut list);

        assert_eq!(
            list,
            vec![
                ((11, 1, 0, 0), GuardEvent::BeginShift(10)),
                ((11, 1, 0, 5), GuardEvent::FallAsleep),
                ((11, 1, 0, 25), GuardEvent::WakeUp),
                ((11, 1, 0, 30), GuardEvent::FallAsleep),
                ((11, 1, 0, 55), GuardEvent::WakeUp),
                ((11, 1, 23, 58), GuardEvent::BeginShift(99)),
                ((11, 2, 0, 40), GuardEvent::FallAsleep),
                ((11, 2, 0, 50), GuardEvent::WakeUp),
                ((11, 3, 0, 5), GuardEvent::BeginShift(10)),
                ((11, 3, 0, 24), GuardEvent::FallAsleep),
                ((11, 3, 0, 29), GuardEvent::WakeUp),
                ((11, 4, 0,  2), GuardEvent::BeginShift(99)),
                ((11, 4, 0, 36), GuardEvent::FallAsleep),
                ((11, 4, 0, 46), GuardEvent::WakeUp),
                ((11, 5, 0, 3), GuardEvent::BeginShift(99)),
                ((11, 5, 0, 45), GuardEvent::FallAsleep),
                ((11, 5, 0, 55), GuardEvent::WakeUp)
            ]
        )
    }

    #[test]
    fn it_finds_sleeping_minutes() {
        let list = vec![
            ((11, 4, 0, 36), GuardEvent::FallAsleep),
            ((11, 1, 0, 30), GuardEvent::FallAsleep),
            ((11, 1, 0, 55), GuardEvent::WakeUp),
            ((11, 2, 0, 40), GuardEvent::FallAsleep),
            ((11, 1, 0, 25), GuardEvent::WakeUp),
            ((11, 3, 0, 29), GuardEvent::WakeUp),
            ((11, 2, 0, 50), GuardEvent::WakeUp),
            ((11, 4, 0,  2), GuardEvent::BeginShift(99)),
            ((11, 5, 0, 55), GuardEvent::WakeUp),
            ((11, 5, 0, 3), GuardEvent::BeginShift(99)),
            ((11, 1, 0, 0), GuardEvent::BeginShift(10)),
            ((11, 5, 0, 45), GuardEvent::FallAsleep),
            ((11, 4, 0, 46), GuardEvent::WakeUp),
            ((11, 1, 0, 5), GuardEvent::FallAsleep),
            ((11, 1, 23, 58), GuardEvent::BeginShift(99)),
            ((11, 3, 0, 24), GuardEvent::FallAsleep),
            ((11, 3, 0, 5), GuardEvent::BeginShift(10)),
        ];

        let res = solve1(list);

        assert_eq!(res, 240)
    }

    #[test]
    fn it_finds_sleeping_minutes_for_input() {
        let strings = map_lines_to_strings("./src/day4/input");

        let list = strings.iter().map(|s| parse_line(s)).collect();

        let res = solve1(list);

        assert_eq!(res, 99911)
    }

    #[test]
    fn it_finds_the_most_frequent_asleep_minute() {
        let list = vec![
            ((11, 4, 0, 36), GuardEvent::FallAsleep),
            ((11, 1, 0, 30), GuardEvent::FallAsleep),
            ((11, 1, 0, 55), GuardEvent::WakeUp),
            ((11, 2, 0, 40), GuardEvent::FallAsleep),
            ((11, 1, 0, 25), GuardEvent::WakeUp),
            ((11, 3, 0, 29), GuardEvent::WakeUp),
            ((11, 2, 0, 50), GuardEvent::WakeUp),
            ((11, 4, 0,  2), GuardEvent::BeginShift(99)),
            ((11, 5, 0, 55), GuardEvent::WakeUp),
            ((11, 5, 0, 3), GuardEvent::BeginShift(99)),
            ((11, 1, 0, 0), GuardEvent::BeginShift(10)),
            ((11, 5, 0, 45), GuardEvent::FallAsleep),
            ((11, 4, 0, 46), GuardEvent::WakeUp),
            ((11, 1, 0, 5), GuardEvent::FallAsleep),
            ((11, 1, 23, 58), GuardEvent::BeginShift(99)),
            ((11, 3, 0, 24), GuardEvent::FallAsleep),
            ((11, 3, 0, 5), GuardEvent::BeginShift(10)),
        ];

        let res = solve2(list);

        assert_eq!(res, 4455)
    }

    #[test]
    fn it_finds_sleeping_minute_for_input_star2() {
        let strings = map_lines_to_strings("./src/day4/input");

        let list = strings.iter().map(|s| parse_line(s)).collect();

        let res = solve2(list);

        assert_eq!(res, 65854)
    }

}
