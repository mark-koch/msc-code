use quizx::circuit::*;
use quizx::gate::*;
use crate::variance;


pub fn intro_example(n : usize) -> Circuit {
    let mut c = Circuit::new(n);

    for i in 0..n {
        c.add_gate("h", vec![i]);
    }

    c.push(Gate::new_with_phase(ParityPhase, (0..n).collect(), variance::parameter()));

    for i in 0..n {
        c.add_gate("h", vec![i]);
    }

    c.push(Gate::new_with_phase(ParityPhase, (0..n).collect(), variance::parameter()));

    return c;
}


pub fn iqp_example() -> Circuit {
    let mut c = Circuit::new(4);

    for i in 0..4 {
        c.add_gate("h", vec![i]);
    }

    c.push(Gate::new_with_phase(ParityPhase, vec![1,2,3], variance::parameter()));
    c.push(Gate::new_with_phase(ParityPhase, vec![0,2,3], variance::parameter()));
    c.push(Gate::new_with_phase(ParityPhase, vec![1,2], variance::parameter()));

    for i in 0..4 {
        c.add_gate("h", vec![i]);
    }

    return c;
}
