use quizx::circuit::*;
use quizx::gate::*;
use crate::variance;


pub fn circ1(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for i in 0..n {
        c.add_gate("h", vec![i]);
    }

    for _ in 0..l {
        c.push(Gate::new_with_phase(ParityPhase, (0..n).collect(), variance::parameter()));

        for i in 0..n {
            c.add_gate("h", vec![i]);
        }
    }

    return c;
}


pub fn circ2(n: usize, l: usize) -> Circuit {
    if n % 2 == 1 {
        panic!("Number of qubits must be even for IQP-2");
    }

    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            c.add_gate("h", vec![i]);
        }
        for i in 0..n {
            if i % 2 == 0 {
                c.push(Gate::new_with_phase(ParityPhase, vec![i,i+1], variance::parameter()));
            }
        }
    }
    for i in 0..n {
        c.add_gate("h", vec![i]);
    }

    return c;
}


pub fn circ3(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for i in 0..n {
        c.add_gate("h", vec![i]);
    }

    for _ in 0..l {
        for i in 0..n-1 {
            c.push(Gate::new_with_phase(ParityPhase, vec![i, i+1], variance::parameter()));
        }

        for j in 0..n {
            c.add_gate("h", vec![j]);
        }
    }

    return c;
}

