use aoc_common::util;

const DECRYPTION_KEY: i64 = 811589153;

pub fn decrypt_table(lines: impl Iterator<Item = String>) -> util::GenericResult<(i64, i64)> {
    let data = lines
        .map(|line| line.parse::<i64>().map(|num| num * DECRYPTION_KEY))
        .collect::<Result<Vec<_>, _>>()?;

    let mut numbers = data.clone();
    let mut order_to_pos = (0..data.len()).collect::<Vec<_>>();
    let mut pos_to_order = (0..data.len()).collect::<Vec<_>>();

    for _ in 0..10 {
        for i in 0..data.len() {
            let pos = order_to_pos[i];
            let number = numbers[pos];

            let new_pos = if number == 0 {
                pos
            } else {
                // First, considering all other elements as static, find out *after which static element*
                // current number should be located before and after the move
                let static_count = numbers.len() - 1;
                let current_static = (pos as i64 - 1).rem_euclid(static_count as i64) as usize;
                let new_static = (current_static as i64 + number).rem_euclid(static_count as i64) as usize;
                // +1 to determine actual position of the new element
                new_static + 1
            };

            match pos.cmp(&new_pos) {
                std::cmp::Ordering::Less => {
                    for p in pos..new_pos {
                        numbers[p] = numbers[p + 1];

                        let order = pos_to_order[p + 1];
                        pos_to_order[p] = order;
                        order_to_pos[order] = p;
                    }
                }
                std::cmp::Ordering::Greater => {
                    for p in ((new_pos + 1)..(pos + 1)).rev() {
                        numbers[p] = numbers[p - 1];

                        let order = pos_to_order[p - 1];
                        pos_to_order[p] = order;
                        order_to_pos[order] = p;
                    }
                }
                std::cmp::Ordering::Equal => {}
            }

            numbers[new_pos] = number;

            order_to_pos[i] = new_pos;
            pos_to_order[new_pos] = i;
        }
    }

    let zero_pos = numbers.iter().position(|num| *num == 0).expect("There should be exactly one element 0");

    let sum = numbers[(zero_pos + 1000) % numbers.len()] + numbers[(zero_pos + 2000) % numbers.len()] + numbers[(zero_pos + 3000) % numbers.len()];

    Ok((0, sum))
}
