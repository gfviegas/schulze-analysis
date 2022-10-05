use schulze::print_graph;

mod schulze;
mod generator;

fn print_title(title: &str) {
    println!("\n\t\t « {title} » :\n");
    for _i in 0..20 { print!("-"); }
    println!("");
}

fn run () {
    let ns = [3,5,7,12,17,22,25,27,32,35,40,43,46];
    let ns = [2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];

    for n in ns {
        print_title(&n.to_string());
        let graph = generator::generate_schulze_matrix(n, 50);
        schulze::schulze_recursive(&graph);
        println!("---");
    }
}

fn test() {
    let graph = generator::generate_graph_2();

    print_title("Iterativo");
    print_graph(&schulze::schulze_iterative(&graph));
    print_title("Recursivo");
    print_graph(&schulze::schulze_recursive(&graph));

    // assert_eq!(iterative_result, recursive_result);
}

fn main () {
    // Spawn thread with explicit stack size
    let child = std::thread::Builder::new()
        .stack_size(7000 * 1024 * 1024)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
