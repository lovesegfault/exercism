use std::cmp::Ordering;
use std::fmt::Write;

use itertools::Itertools;

const WIN: usize = 3;
const LOSS: usize = 0;
const DRAW: usize = 1;

pub fn tally(match_results: &str) -> String {
    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");

    match_results
        .lines()
        .map(|l| l.split_terminator(';').collect::<Vec<_>>())
        .map(|l| (l[0], l[1], l[2]))
        .filter_map(|(p0, p1, res)| match res {
            "win" => Some([(p0, WIN), (p1, LOSS)]),
            "draw" => Some([(p0, DRAW), (p1, DRAW)]),
            "loss" => Some([(p0, LOSS), (p1, WIN)]),
            _ => None,
        })
        .flat_map(std::array::IntoIter::new)
        .into_group_map()
        .into_iter()
        .map(|(name, results)| {
            (
                name,
                results
                    .into_iter()
                    .fold((0, 0, 0, 0, 0), |mut tally, result| {
                        tally.0 += 1; // matches played
                        match result {
                            WIN => {
                                tally.1 += 1; // wins
                                tally.4 += WIN; // points
                            }
                            DRAW => {
                                tally.2 += 1; // draws
                                tally.4 += DRAW; // points
                            }
                            LOSS => {
                                tally.3 += 1; // losses
                            }
                            _ => unreachable!(),
                        }
                        tally
                    }),
            )
        })
        .map(|(name, (mp, w, d, l, p))| (name, mp, w, d, l, p))
        .sorted_unstable_by(|(name_a, _, _, _, _, p_a), (name_b, _, _, _, _, p_b)| {
            match p_a.cmp(p_b) {
                Ordering::Equal => name_a.cmp(name_b),
                o => o.reverse(),
            }
        })
        .for_each(|(name, mp, w, d, l, p)| {
            write!(
                &mut result,
                "\n{:<30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                name, mp, w, d, l, p,
            )
            .ok();
        });

    result
}
