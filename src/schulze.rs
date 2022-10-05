use std::i32;

/// Returns the maximum value between two integers
fn max(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b };
}

/// Returns the minimum value between two integers
fn min(a: i32, b: i32) -> i32 {
    return if a < b { a } else { b };
}

/// Cleanses the graph matrix, keeping only the edges which has a greater value than the reverse directed edge.
///
/// Returns a new matrix with 0's except with the cells which it counterpart has a smaller value.
pub fn prepare_graph(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let contestants_amount: usize = (*graph).len();
    let mut strong_links: Vec<Vec<i32>> = vec![vec![0; contestants_amount]; contestants_amount];

    for i in 0..contestants_amount {
        for j in 0..contestants_amount {
            if i == j { continue; }

            if (*graph)[i][j] > (*graph)[j][i] {
                strong_links[i][j] = (*graph)[i][j];
            }
        }
    }

    return strong_links;
}

/// Prints a graph matrix.
pub fn print_graph(graph: &Vec<Vec<i32>>) {
    let pad: usize = 2;

    for i in 0..graph.len() {
        print!("| ");
        for j in 0..graph.len() {
            if graph[i][j] == 0 {
                print!("{:01$} ", "  ", pad);
                continue;
            }

            print!("{:01$} ", graph[i][j], pad);
        }
        print!("|\n");
    }

    print!("\n\n");
}

/// With a Schulze output result, calculates how many wins each candidate had, printing the results and returning
/// the amount of wins each i-th candidate had.
pub fn rank_candidates(strong_links: &Vec<Vec<i32>>) {
    let contestants_amount: usize = (*strong_links).len();
    let mut contestants_wins: Vec<i32> = vec![0;  contestants_amount];

    for i in 0..contestants_amount {
        for j in 0..contestants_amount {
            if (*strong_links)[i][j] > (*strong_links)[j][i] {
                contestants_wins[i] += 1;
            }
        }

        println!("Candidate {} had {} wins.", i, contestants_wins[i]);
    }
}

/// Iterative version of Schulze algorithm.
pub fn schulze_iterative(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let contestants_amount: usize = (*graph).len();
    let mut strong_links: Vec<Vec<i32>> = prepare_graph(graph);
    let mut comparisons: u32 = 0;

    println!("\tInput: ");
    print_graph(graph);
    println!("\tPrepared Graph: ");
    print_graph(&strong_links);

    let start = std::time::Instant::now();
    for i in 0..contestants_amount {
        for j in 0..contestants_amount {
            if i == j { continue; }

            for k in 0..contestants_amount {
                comparisons += 4;
                if !(i != k && j != k) { continue; }

                let weakest_edge = min(strong_links[j][i], strong_links[i][k]);
                let strength = max(strong_links[j][k], weakest_edge);

                strong_links[j][k] = strength;

            }
        }
    }
    let duration = start.elapsed();

    println!("\tResult: ");
    print_graph(&strong_links);
    rank_candidates(&strong_links);

    println!("{};{};{:?}", contestants_amount, comparisons, duration.as_secs_f64());

    return strong_links;
}

pub fn schulze_recursive_step(strong_links: &mut Vec<Vec<i32>>, comparisons: &mut u32, j: usize, k: usize, i: usize) -> i32 {
    // Base Condition
    *comparisons += 1;
    if i == 0 {
        return strong_links[j][k];
    }

    // Diagonal
    *comparisons += 1;
    if k == j   {
        return 0;
    }

    *comparisons += 2;
    let weakest_edge = min(schulze_recursive_step(strong_links, comparisons, j, i, i-1), schulze_recursive_step(strong_links, comparisons, i, k, i-1));
    let strength = max(schulze_recursive_step(strong_links, comparisons, j, k, i-1), weakest_edge);

    return strength;
}


/// Recursive version of Schulze algorithm.
pub fn schulze_recursive(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let contestants_amount: usize = (*graph).len();
    let mut strong_links: Vec<Vec<i32>> = prepare_graph(graph);

    println!("\tInput: ");
    print_graph(graph);
    println!("\tPrepared Graph: ");
    print_graph(&strong_links);

    let mut comparisons: u32 = 0;
    let start = std::time::Instant::now();
    for a in (0..contestants_amount).rev() {
        for b in (0..contestants_amount).rev() {
            strong_links[a][b] = schulze_recursive_step(&mut strong_links, &mut comparisons, a, b, contestants_amount - 1);
        }
    }
    let duration = start.elapsed();

    println!("\tResult: ");
    print_graph(&strong_links);
    rank_candidates(&strong_links);

    println!("{};{};{:?}", contestants_amount, comparisons, duration.as_secs_f64());

    return strong_links;
}

