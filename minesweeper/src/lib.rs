pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut changes = Vec::new();
    let mut result: Vec<String> = minefield.iter().map(|&s| s.to_string()).collect();

    for (i, row) in minefield.iter().enumerate() {
        for (j, chr) in row.bytes().enumerate() {
            if chr != b'*' {
                continue;
            }

            // row above
            if i as i32 > 0 {
                handle_row(&mut changes, row, i - 1, j);
            }

            // current row
            handle_row(&mut changes, row, i, j);

            // row bellow
            if i + 1 < minefield.len() {
                handle_row(&mut changes, row, i + 1, j);
            }
        }
    }

    for position in changes {
        let row: &mut str = result[position.0].as_mut();
        let chr = row.as_bytes()[position.1];
        if chr == b'*' {
            continue;
        }

        let modified_chr = match chr {
            b' ' => b'1',
            _ => chr + 1,
        };

        let transformed_row: Vec<u8> = row
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, &b)| if i == position.1 { modified_chr } else { b })
            .collect();

        result[position.0] = String::from_utf8(transformed_row.to_owned()).unwrap();
    }

    result
}

fn handle_row(changes: &mut Vec<(usize, usize)>, row: &&str, i: usize, j: usize) {
    changes.push((i, j));

    if (j as i32 - 1) >= 0 {
        changes.push((i, j - 1));
    }

    if (j + 1) < row.len() {
        changes.push((i, j + 1));
    }
}
