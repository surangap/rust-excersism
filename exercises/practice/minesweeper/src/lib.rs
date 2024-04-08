use std::str;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    minefield.iter().enumerate().for_each(|(r, &row)| {
        // let mut result_row: String = "".to_string();
        let mut result_row: Vec<u8> = vec![];
        row.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(c, &syl)| match syl {
                42 => result_row.push(syl),
                _ => result_row.push(find_mine_count(r, c, minefield)),
            });
        result.push(String::from_utf8(result_row).expect("should be valid ascii"));
    });
    print!("{:?}", result);
    result
}

/// Iteration 1
// fn find_mine_count(r: usize, c: usize, minefield: &[&str]) -> u8 {
//     let mut count = 0_u8;
//     let mf_tot_rows = minefield.len();
//     let mf_tot_cols = minefield[0].as_bytes().len();
//     assert!(r < mf_tot_rows);
//     assert!(c < mf_tot_cols);
//     assert_ne!(minefield[r].as_bytes()[c], "*".as_bytes()[0]);
// 
//     for ri in r.saturating_sub(1)..=r + 1 {
//         for ci in c.saturating_sub(1)..=c + 1 {
//             if (ri == r && ci == c) || ri >= mf_tot_rows || ci >= mf_tot_cols {
//                 continue;
//             }
//             if minefield[ri].as_bytes()[ci] == "*".as_bytes()[0] {
//                 count += 1;
//             }
//         }
//     }
//     if count > 0 {
//         count.to_string().as_bytes()[0]
//     } else {
//         " ".as_bytes()[0]
//     }
// }

/// Iteration 2
const window:&[&[(i32, i32)]] = &[
    &[(-1,-1), (-1, 0), (-1,1)],
    &[(0,-1), (0,0), (0,1)],
    &[(1,-1), (1, 0), (1,1)],
];
fn find_mine_count(r: usize, c: usize, minefield: &[&str]) -> u8 {
    let mut count = 0;
    let mf_tot_rows = minefield.len();
    let mf_tot_cols = minefield[0].as_bytes().len();
    //"1*2*1"
    window.iter()
        .for_each(|&row| {
            count += row.iter()
                .map(|&(y, x)| {
                    (y+r as i32, x+c as i32)
                })
                .filter(|&(y, x)| {
                    (y >= 0 && y < mf_tot_rows as i32) && (x >= 0 && x < mf_tot_cols as i32)
                })
                .filter(|&(y, x)| {
                    minefield[y as usize].as_bytes()[x as usize] == b'*'
                })
                .count();
        });
    
    match count {
        0 => b' ',
        n => n as u8 + '0' as u8
    }
}