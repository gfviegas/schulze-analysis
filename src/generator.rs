use std::i32;
use rand::Rng;

pub fn generate_schulze_matrix(contestants_amount: usize, voters_amount: i32) -> Vec<Vec<i32>> {
    let mut graph: Vec<Vec<i32>> = vec![vec![0; contestants_amount]; contestants_amount];

    for i in 0..contestants_amount {
        for j in i..contestants_amount {
            if i == j { continue; }

            let random_score = rand::thread_rng().gen_range(1..voters_amount);
            graph[i][j] = random_score;
            graph[j][i] = voters_amount - random_score;
        }
    }

    return graph;
}

pub fn generate_graph_1() -> Vec<Vec<i32>> {
    return vec![
        vec![0, 20, 26, 30, 22],
        vec![25, 0, 16, 33, 18],
        vec![19, 29, 0, 17, 24],
        vec![15, 12, 28, 0, 14],
        vec![23, 27, 21, 31, 0],
    ];
}

pub fn generate_graph_2() -> Vec<Vec<i32>> {
    return vec![
        vec![0, 15, 9, 23, 40],
        vec![35, 0, 24, 31, 22],
        vec![41, 26, 0, 23, 40],
        vec![27, 19, 27, 0, 35],
        vec![10, 28, 10, 15, 0],
    ];
}
