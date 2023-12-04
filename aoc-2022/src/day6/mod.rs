use aoc_common::util;

fn find_first_marker<const N: usize>(line: &String) -> util::GenericResult<usize> {
    for (num, window) in line.as_bytes().windows(N).enumerate() {
        // check that all elements in the window are unique
        let set: util::BitSet = window.iter().map(|ch| (ch - b'a') as u64).collect();
        if set.len() == N {
            return Ok(num + N);
        }
    }

    Err("Could not find any markers in the input".into())
}

pub fn find_markers<const P: usize, const M: usize>(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let line = lines.next().ok_or("Input is empty")?;

    Ok((find_first_marker::<P>(&line)?, find_first_marker::<M>(&line)?))
}
