pub mod sim;
pub mod variance;
pub mod iqp;
pub mod toy;

use std::env;
use quizx::circuit::*;


fn get_ansatz(name: &String, n: usize, l: usize) -> Circuit {
    match name.as_str() {
        "sim1" => sim::circ1(n, l),
        "sim2" => sim::circ2(n, l),
        "sim9" => sim::circ9(n, l),
        "sim10" => sim::circ10(n, l),
        "sim11" => sim::circ11(n, l),
        "sim12" => sim::circ12(n, l),
        "sim15" => sim::circ15(n, l),
        "iqp1" => iqp::circ1(n, l),
        "iqp2" => iqp::circ2(n, l),
        "iqp3" => iqp::circ3(n, l),
        "introExample" => toy::intro_example(n),
        "iqpExample" => toy::iqp_example(),
        _ => panic!("Circuit not found: {name}")
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let ansatz = &args[1];
    let qubits: usize = args[2].parse().unwrap();
    let layers: usize = args[3].parse().unwrap();
    let h = &args[4];

    let mut param = 0;
    if args.len() > 5 {
        param = args[5].parse().unwrap();
    }

    let mut c = get_ansatz(ansatz, qubits, layers);
    variance::select_target(&mut c, param);

    let var = variance::compute_variance(&mut c, &h);
    println!("{var}");

}
