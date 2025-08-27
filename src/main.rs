use std::{collections::HashSet, fmt::Display, mem::replace, usize};

use clap::Parser;
use rand::{
    Rng, rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser=clap::value_parser!(u8).range(2..4+1))]
    dimension: u8,
}

struct Test<T: ToString>(Vec<Vec<T>>);

impl<T> Display for Test<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str("[");

        let n = self.0.len();

        for i in 0..n {
            out.push_str("[");
            for j in 0..n {
                out.push_str(&self.0[i][j].to_string());
                if j != n - 1 {
                    out.push_str(", ");
                }
            }
            out.push_str("]");
            if i != n - 1 {
                out.push_str(", ");
            }
        }

        out.push_str("]");
        write!(f, "{}", out)
    }
}

fn gen_test(dimension: usize) -> (Vec<Vec<String>>, Vec<Vec<usize>>) {
    let mut matrix = vec![];
    let mut rng = rng();
    let diagonal = rng.random_range(1..9 + 1);

    // calculate row totals
    for i in 0..dimension {
        matrix.push(gen_row(i, diagonal, dimension, &mut rng));
    }

    // calculate column totals
    let mut transposed: Vec<Vec<usize>> = transpose(&mut matrix)
        .into_iter()
        .map(|mut row| {
            row.insert(0, random_total(&row, &mut rng));
            row
        })
        .collect();

    // top left corner can be ignored, but set to 0 anyway
    let _ = replace(&mut transposed[0][0], 0);

    (mask(&transposed), transposed)
}

fn valid_test(transposed: &Vec<Vec<usize>>) -> bool {
    for row in transposed {
        let set: HashSet<&usize> = HashSet::from_iter(row);
        if set.len() != row.len() {
            return false;
        }
    }

    true
}

fn mask(matrix: &Vec<Vec<usize>>) -> Vec<Vec<String>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| match row {
                    0 => matrix[row][col].to_string(),
                    _ => match col {
                        0 => matrix[row][col].to_string(),
                        _ => "_".to_string(),
                    }
                    .to_string(),
                })
                .collect()
        })
        .collect()
}

fn transpose(matrix: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
        .collect()
}

fn gen_row(index: usize, diagonal: usize, dimension: usize, rng: &mut ThreadRng) -> Vec<usize> {
    // fill row with random data from 1-9
    let mut numbers: Vec<usize> = (1..10).collect();

    numbers.shuffle(rng);

    let mut row: Vec<usize> = numbers[0..dimension].to_vec();

    // set pre-generated random diagonal value
    let _ = replace(&mut row[index], diagonal);

    // prepend sum or product
    row.insert(0, random_total(&row, rng));

    row
}

fn random_total(row: &Vec<usize>, rng: &mut ThreadRng) -> usize {
    let totals = [row.iter().sum(), row.iter().product()];
    *totals.choose(rng).unwrap()
}

fn main() {
    let args = Args::parse();
    let (mut masked, mut unmasked) = gen_test(args.dimension.into());

    while !valid_test(&unmasked) {
        (masked, unmasked) = gen_test(args.dimension.into());
    }

    println!("Puzzle = {}, puzzle_solution(Puzzle).", Test(masked));
    println!("Puzzle = {}", Test(unmasked));
}
