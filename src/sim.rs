use quizx::circuit::*;
use num::rational::Rational;
use crate::variance;


pub fn circ1(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            c.add_gate_with_phase("rx", vec![i], variance::parameter());
            c.add_gate_with_phase("rz", vec![i], variance::parameter());
        }
    }

    return c;
}


pub fn circ2(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            c.add_gate_with_phase("rx", vec![i], variance::parameter());
            c.add_gate_with_phase("rz", vec![i], variance::parameter());
        }
        let mut i = n-1;
        while i > 0 {
            c.add_gate("cx", vec![i, i-1]);
            i -= 1
        }
    }

    return c;
}


pub fn circ9(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            c.add_gate("h", vec![i]);
        }
        for i in 0..n-1 {
            c.add_gate("cz", vec![i, i+1]);
        }
        for i in 0..n {
            c.add_gate_with_phase("rx", vec![i], variance::parameter());
        }
    }
    
    return c;
}

fn add_ry(c: &mut Circuit, i: usize, phase: Rational) {
    c.add_gate_with_phase("rz", vec![i], Rational::new(-1, 2));
    c.add_gate_with_phase("rx", vec![i], phase);
    c.add_gate_with_phase("rz", vec![i], Rational::new(1, 2));
}


pub fn circ10(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for i in 0..n {
        add_ry(&mut c, i, variance::parameter());
    }

    for _ in 0..l {
        for i in 0..n-1 {
            c.add_gate("cz", vec![i, i+1]);
        }
        c.add_gate("cz", vec![0, n-1]);
        for i in 0..n {
            add_ry(&mut c, i, variance::parameter());
        }
    }
    
    return c;
}


pub fn circ11(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            add_ry(&mut c, i, variance::parameter());
            c.add_gate_with_phase("rz", vec![i], variance::parameter());
        }

        let mut i = 0;
        while i < n-1 {
            c.add_gate("cx", vec![i, i+1]);
            i += 2;
        }

        let mut i = 1;
        while i < n {
            c.add_gate_with_phase("ry", vec![i], variance::parameter());
            c.add_gate_with_phase("rz", vec![i], variance::parameter());
            if i+1 < n {
                add_ry(&mut c, i+1, variance::parameter());
                c.add_gate_with_phase("rz", vec![i+1], variance::parameter());
                c.add_gate("cx", vec![i, i+1]);
            }
            i += 3;
        }
    }
    
    return c;
}


pub fn circ12(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            add_ry(&mut c, i, variance::parameter());
            c.add_gate_with_phase("rz", vec![i], variance::parameter());
        }

        let mut i = 0;
        while i < n-1 {
            c.add_gate("cz", vec![i, i+1]);
            i += 2;
        }

        let mut i = 1;
        while i < n {
            c.add_gate_with_phase("ry", vec![i], variance::parameter());
            c.add_gate_with_phase("rz", vec![i], variance::parameter());
            if i+1 < n {
                add_ry(&mut c, i+1, variance::parameter());
                c.add_gate_with_phase("rz", vec![i+1], variance::parameter());
                c.add_gate("cz", vec![i, i+1]);
            }
            i += 3;
        }


    }
    
    return c;
}


pub fn circ15(n: usize, l: usize) -> Circuit {
    let mut c = Circuit::new(n);

    for _ in 0..l {
        for i in 0..n {
            add_ry(&mut c, i, variance::parameter());
        }

        c.add_gate("cx", vec![0, n-1]);

        let mut i = n-1;
        while i > 0 {
            c.add_gate("cx", vec![i, i-1]);
            i -= 1;
        }

        for i in 0..n {
            add_ry(&mut c, i, variance::parameter());
        }

        c.add_gate("cx", vec![n-2, n-1]);
        c.add_gate("cx", vec![n-1, 0]);
        for i in 1..n-1 {
            c.add_gate("cx", vec![i-1, i]);
        }
    }
    
    return c;
}

