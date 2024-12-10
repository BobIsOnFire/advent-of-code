use aoc_common::util;

fn compress_blocks(files: &[u8], gaps: &[u8]) -> usize {
    let mut data = vec![];
    for (id, (&file_len, &gap_len)) in files.iter().zip(gaps).enumerate() {
        data.extend(std::iter::repeat(Some(id)).take(file_len.into()));
        data.extend(std::iter::repeat(None).take(gap_len.into()));
    }

    let mut gap_idx = 0;
    let mut file_idx = data.len() - 1;

    loop {
        while data[gap_idx].is_some() {
            gap_idx += 1;
        }
        while data[file_idx].is_none() {
            file_idx -= 1;
        }
        if gap_idx >= file_idx {
            break;
        }

        data.swap(gap_idx, file_idx);
    }

    assert!(data.iter().skip_while(|v| v.is_some()).all(Option::is_none));

    data.into_iter()
        .flatten()
        .enumerate()
        .map(|(position, id)| position * id)
        .sum()
}

#[derive(Debug)]
struct Block {
    id: usize,
    size: usize,
    position: usize,
}

impl Block {
    fn get_checksum(&self) -> usize {
        self.id * (self.position..(self.position + self.size)).sum::<usize>()
    }
}

fn compress_whole_files(files: &[u8], gaps: &[u8]) -> usize {
    let mut current_position = 0;

    let mut file_blocks = vec![];
    let mut gap_blocks = vec![];

    for (id, (&file_len, &gap_len)) in files.iter().zip(gaps).enumerate() {
        let file = Block {
            id,
            size: file_len.into(),
            position: current_position,
        };
        current_position += file.size;
        file_blocks.push(file);

        let gap = Block {
            id,
            size: gap_len.into(),
            position: current_position,
        };
        current_position += gap.size;
        gap_blocks.push(gap);
    }

    for file in file_blocks.iter_mut().rev() {
        if let Some(gap) = gap_blocks
            .iter_mut()
            .take_while(|g| g.position < file.position)
            .find(|g| file.size <= g.size)
        {
            file.position = gap.position;

            gap.position += file.size;
            gap.size -= file.size;
        }
    }

    file_blocks.iter().map(Block::get_checksum).sum()
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let line = lines.into_iter().next().ok_or("Input is empty")?;

    let files = line
        .bytes()
        .enumerate()
        .filter_map(|(idx, byte)| (idx % 2 == 0).then_some(byte - b'0'))
        .collect::<Vec<_>>();

    let gaps = line
        .bytes()
        .enumerate()
        .filter_map(|(idx, byte)| (idx % 2 == 1).then_some(byte - b'0'))
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    assert!(files.len() == gaps.len());

    let block_checksum = compress_blocks(&files, &gaps);
    let file_checksum = compress_whole_files(&files, &gaps);

    Ok((block_checksum, file_checksum))
}
