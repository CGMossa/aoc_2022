use itertools::Itertools;
use std::collections::VecDeque;

type Worry = u128;

struct Monkey {
    /// items and their worry level
    worry: VecDeque<Worry>,
    operation: Box<dyn Fn(Worry) -> Worry>,
    test: Box<dyn Fn(Worry) -> usize>,
    divisible_by: Worry,
}

impl Monkey {
    fn operation(&self, old: Worry) -> Worry {
        (self.operation)(old)
    }
    fn test(&self, worry: Worry) -> usize {
        (self.test)(worry)
    }
}

fn main() {
    println!("Answer: {}", one_star_example());
    println!("Answer: {}", one_star());
    println!("Answer: {}", two_star_example());
    println!("Answer: {}", two_star());
}

fn one_star_example() -> usize {
    let mut monkeys = [
        // 0
        Monkey {
            worry: [79, 98].into(),
            operation: Box::new(|old| old * 19),
            divisible_by: 23,
            test: Box::new(|worry| if worry % 23 == 0 { 2 } else { 3 }),
        },
        // 1
        Monkey {
            worry: [54, 65, 75, 74].into(),
            operation: Box::new(|old| old + 6),
            divisible_by: 19,
            test: Box::new(|worry| if worry % 19 == 0 { 2 } else { 0 }),
        },
        // 2
        Monkey {
            worry: [79, 60, 97].into(),
            operation: Box::new(|old| old * old),
            divisible_by: 13,
            test: Box::new(|worry| if worry % 13 == 0 { 1 } else { 3 }),
        },
        // 3
        Monkey {
            worry: [74].into(),
            operation: Box::new(|old| old + 3),
            divisible_by: 17,
            test: Box::new(|worry| if worry % 17 == 0 { 0 } else { 1 }),
        },
    ];
    let mut inspections = vec![0; monkeys.len()];
    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let items = monkeys[monkey_id].worry.drain(0..).collect_vec();
            for item in items {
                let monkey = &mut monkeys[monkey_id];
                let item = monkey.operation(item);
                let item = item / 3;
                let next_monkey = monkey.test(item);
                monkeys[next_monkey].worry.push_back(item);
                inspections[monkey_id] += 1;
            }
        }
        for (id, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", id, monkey.worry);
        }
        println!();
    }

    for (id, total_inspections) in inspections.iter().enumerate() {
        println!("Monkey {id} inspected items {total_inspections} times.");
    }
    println!();

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn two_star_example() -> Worry {
    let mut monkeys = [
        // 0
        Monkey {
            worry: [79, 98].into(),
            operation: Box::new(|old| old * 19),
            divisible_by: 23,
            test: Box::new(|worry| if worry % 23 == 0 { 2 } else { 3 }),
        },
        // 1
        Monkey {
            worry: [54, 65, 75, 74].into(),
            operation: Box::new(|old| old + 6),
            divisible_by: 19,
            test: Box::new(|worry| if worry % 19 == 0 { 2 } else { 0 }),
        },
        // 2
        Monkey {
            worry: [79, 60, 97].into(),
            operation: Box::new(|old| old * old),
            divisible_by: 13,
            test: Box::new(|worry| if worry % 13 == 0 { 1 } else { 3 }),
        },
        // 3
        Monkey {
            worry: [74].into(),
            operation: Box::new(|old| old + 3),
            divisible_by: 17,
            test: Box::new(|worry| if worry % 17 == 0 { 0 } else { 1 }),
        },
    ];
    let common_divisor: Worry = monkeys.iter().map(|x| x.divisible_by).product();
    let mut inspections = vec![0; monkeys.len()];
    for _round in 0..10_000 {
        for monkey_id in 0..monkeys.len() {
            let items = monkeys[monkey_id].worry.drain(0..).collect_vec();
            for item in items {
                let monkey = &mut monkeys[monkey_id];
                let item = monkey.operation(item);
                // let item = item / 3;
                let item = item % common_divisor;
                let next_monkey = monkey.test(item);
                monkeys[next_monkey].worry.push_back(item);
                inspections[monkey_id] += 1;
            }
        }
        // for (id, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {:?}", id, monkey.worry);
        // }
        // println!();
    }

    for (id, total_inspections) in inspections.iter().enumerate() {
        println!("Monkey {id} inspected items {total_inspections} times.");
    }
    println!();

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn one_star() -> usize {
    let mut monkeys = [
        Monkey {
            worry: [66, 71, 94].into(),
            operation: Box::new(|old| old * 5),
            divisible_by: 3,
            test: Box::new(|worry| if worry % 3 == 0 { 7 } else { 4 }),
        },
        Monkey {
            worry: [70].into(),
            operation: Box::new(|old| old + 6),
            divisible_by: 17,
            test: Box::new(|worry| if worry % 17 == 0 { 3 } else { 0 }),
        },
        Monkey {
            worry: [62, 68, 56, 65, 94, 78].into(),
            operation: Box::new(|old| old + 5),
            divisible_by: 2,
            test: Box::new(|worry| if worry % 2 == 0 { 3 } else { 1 }),
        },
        Monkey {
            worry: [89, 94, 94, 67].into(),
            operation: Box::new(|old| old + 2),
            divisible_by: 19,
            test: Box::new(|worry| if worry % 19 == 0 { 7 } else { 0 }),
        },
        Monkey {
            worry: [71, 61, 73, 65, 98, 98, 63].into(),
            operation: Box::new(|old| old * 7),
            divisible_by: 11,
            test: Box::new(|worry| if worry % 11 == 0 { 5 } else { 6 }),
        },
        Monkey {
            worry: [55, 62, 68, 61, 60].into(),
            operation: Box::new(|old| old + 7),
            divisible_by: 5,
            test: Box::new(|worry| if worry % 5 == 0 { 2 } else { 1 }),
        },
        Monkey {
            worry: [93, 91, 69, 64, 72, 89, 50, 71].into(),
            operation: Box::new(|old| old + 1),
            divisible_by: 13,
            test: Box::new(|worry| if worry % 13 == 0 { 5 } else { 2 }),
        },
        Monkey {
            worry: [76, 50].into(),
            operation: Box::new(|old| old * old),
            divisible_by: 7,
            test: Box::new(|worry| if worry % 7 == 0 { 4 } else { 6 }),
        },
    ];
    let mut inspections = vec![0; monkeys.len()];
    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let items = monkeys[monkey_id].worry.drain(0..).collect_vec();
            for item in items {
                let monkey = &mut monkeys[monkey_id];
                let item = monkey.operation(item);
                let item = item / 3;
                let next_monkey = monkey.test(item);
                monkeys[next_monkey].worry.push_back(item);
                inspections[monkey_id] += 1;
            }
        }
        // for (id, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {:?}", id, monkey.worry);
        // }
        // println!();
    }

    for (id, total_inspections) in inspections.iter().enumerate() {
        println!("Monkey {id} inspected items {total_inspections} times.");
    }
    println!();

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn two_star() -> Worry {
    let mut monkeys = [
        Monkey {
            worry: [66, 71, 94].into(),
            operation: Box::new(|old| old * 5),
            divisible_by: 3,
            test: Box::new(|worry| if worry % 3 == 0 { 7 } else { 4 }),
        },
        Monkey {
            worry: [70].into(),
            operation: Box::new(|old| old + 6),
            divisible_by: 17,
            test: Box::new(|worry| if worry % 17 == 0 { 3 } else { 0 }),
        },
        Monkey {
            worry: [62, 68, 56, 65, 94, 78].into(),
            operation: Box::new(|old| old + 5),
            divisible_by: 2,
            test: Box::new(|worry| if worry % 2 == 0 { 3 } else { 1 }),
        },
        Monkey {
            worry: [89, 94, 94, 67].into(),
            operation: Box::new(|old| old + 2),
            divisible_by: 19,
            test: Box::new(|worry| if worry % 19 == 0 { 7 } else { 0 }),
        },
        Monkey {
            worry: [71, 61, 73, 65, 98, 98, 63].into(),
            operation: Box::new(|old| old * 7),
            divisible_by: 11,
            test: Box::new(|worry| if worry % 11 == 0 { 5 } else { 6 }),
        },
        Monkey {
            worry: [55, 62, 68, 61, 60].into(),
            operation: Box::new(|old| old + 7),
            divisible_by: 5,
            test: Box::new(|worry| if worry % 5 == 0 { 2 } else { 1 }),
        },
        Monkey {
            worry: [93, 91, 69, 64, 72, 89, 50, 71].into(),
            operation: Box::new(|old| old + 1),
            divisible_by: 13,
            test: Box::new(|worry| if worry % 13 == 0 { 5 } else { 2 }),
        },
        Monkey {
            worry: [76, 50].into(),
            operation: Box::new(|old| old * old),
            divisible_by: 7,
            test: Box::new(|worry| if worry % 7 == 0 { 4 } else { 6 }),
        },
    ];
    let mut inspections = vec![0; monkeys.len()];
    let common_divisor: Worry = monkeys.iter().map(|x|x.divisible_by).product();
    for _round in 0..10_000 {
        for monkey_id in 0..monkeys.len() {
            let items = monkeys[monkey_id].worry.drain(0..).collect_vec();
            for item in items {
                let monkey = &mut monkeys[monkey_id];
                let item = monkey.operation(item);
                let item = item % common_divisor;
                let next_monkey = monkey.test(item);
                monkeys[next_monkey].worry.push_back(item);
                inspections[monkey_id] += 1;
            }
        }
        // for (id, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {:?}", id, monkey.worry);
        // }
        // println!();
    }

    for (id, total_inspections) in inspections.iter().enumerate() {
        println!("Monkey {id} inspected items {total_inspections} times.");
    }
    println!();

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}
