use quizx::circuit::*;
use quizx::graph::*;
use quizx::scalar::*;
use quizx::hash_graph::*;
use num::rational::Rational;
use rustc_hash::FxHashMap;


pub fn parameter() -> Rational { return Rational::new(1, 42); }
pub fn target() -> Rational { return Rational::new(1, 1337); }


/// Plug the given graph into the outputs and multiply scalars.
/// Same as the quizx implementation but also returns the rename maop.
///
/// Panics if the outputs of `self` are not the same length as the inputs of `other`.
fn plug_vmap(g: &mut Graph, other: &impl GraphLike)-> FxHashMap<V,V>  {
    if other.inputs().len() != g.outputs().len() {
        panic!("Outputs and inputs must match");
    }

    let vmap = g.append_graph(other);

    for k in 0..g.outputs().len() {
        let o = g.outputs()[k];
        let i = other.inputs()[k];
        let (no, et0) = g.incident_edges(o).next().expect(&format!("Bad output: {}", o));
        let (ni, et1) = other.incident_edges(i).next().expect(&format!("Bad input: {}", i));
        let et = EType::merge(et0, et1);

        g.add_edge_smart(no, vmap[&ni], et);
        g.remove_vertex(o);
        g.remove_vertex(vmap[&i]);
    }

    let outp = other.outputs()
        .iter().map(|o| vmap[o]).collect();
    g.set_outputs(outp);

    return vmap;
}


// fn triangle2() -> Graph {
//     let mut g = Graph::new();

//     let input = g.add_vertex(VType::B);
//     let v1 = g.add_vertex_with_phase(VType::X, Rational::new(1,2));
//     let v2 = g.add_vertex_with_phase(VType::Z, Rational::new(1,4));
//     let v3 = g.add_vertex(VType::X);
//     let v4 = g.add_vertex_with_phase(VType::Z, Rational::new(1,4));
//     let v5 = g.add_vertex(VType::Z);
//     let v6 = g.add_vertex(VType::X);
//     let v7 = g.add_vertex_with_phase(VType::Z, Rational::new(-1,4));
//     let v8 = g.add_vertex_with_phase(VType::Z, Rational::new(-1,4));
//     let output = g.add_vertex(VType::B);

//     g.add_edge(input, v1);
//     g.add_edge(v1, v2);
//     g.add_edge(v2, v3);
//     g.add_edge(v3, v4);
//     g.add_edge(v3, v5);
//     g.add_edge(v5, output);
//     g.add_edge(v5, v6);
//     g.add_edge(v6, v7);
//     g.add_edge(v6, v8);

//     g.set_inputs(vec![input]);
//     g.set_outputs(vec![output]);

//     return g;
// }


fn triangle() -> Graph {
    let mut g = Graph::new();

    let input = g.add_vertex(VType::B);
    let v1 = g.add_vertex(VType::X);
    let v2 = g.add_vertex_with_phase(VType::Z, Rational::new(-1,4));
    let v3 = g.add_vertex_with_phase(VType::Z, Rational::new(1,4));
    let v4 = g.add_vertex(VType::X);
    let v5 = g.add_vertex(VType::X);
    let v6 = g.add_vertex_with_phase(VType::Z, Rational::new(-1,4));
    let v7 = g.add_vertex_with_phase(VType::Z, Rational::new(1,4));
    let v8 = g.add_vertex(VType::Z);
    let output = g.add_vertex(VType::B);

    g.add_edge(input, v1);
    g.add_edge(v1, v2);
    g.add_edge(v1, v3);
    g.add_edge(v2, v4);
    g.add_edge(v3, v5);
    g.add_edge(v4, v6);
    g.add_edge(v4, v8);
    g.add_edge(v5, v7);
    g.add_edge(v5, v8);
    g.add_edge(v8, output);

    g.set_inputs(vec![input]);
    g.set_outputs(vec![output]);

    g.scalar_mut().mul_sqrt2_pow(1);

    return g;
}


fn hamiltonian(paulis: &str) -> Graph {
    let mut h = Graph::new();

    for p in paulis.chars() {
        let input = h.add_vertex(VType::B);
        let output = h.add_vertex(VType::B);

        if p == 'I' {
            h.add_edge(input, output);
        } else if p == 'Z' || p == 'X' {
            let v = h.add_vertex_with_phase(if p == 'X' { VType::X } else { VType::Z }, Rational::new(1,1));
            h.add_edge(input, v);
            h.add_edge(v, output);
        } else if p == 'Y' {
            let v1 = h.add_vertex_with_phase(VType::Z, Rational::new(1,1));
            let v2 = h.add_vertex_with_phase(VType::X, Rational::new(1,1));
            h.add_edge(input, v1);
            h.add_edge(v1, v2);
            h.add_edge(v2, output);
            h.scalar_mut().mul_phase(Rational::new(1,2));
        } else {
            panic!("Ivalid Pauli: '{p}'. Must be 'X', 'Y', 'Z', or 'I'");
        }

        h.inputs_mut().push(input);
        h.outputs_mut().push(output);
    }

    return h;
}


fn create_diagram(circ: &Circuit, pauli_hamiltonian: &str) -> Graph {
    if circ.num_qubits() != pauli_hamiltonian.len() {
        panic!("Pauli string has invalid length!");
    }

    let mut g: Graph = circ.to_graph();

    // Find parameterised spiders and fuse them out
    let mut param_verts: Vec<V> = Vec::new();
    let mut target_vert: Option<V> = None;
    for v in g.clone().vertices() {  // TODO: How to iterate without clone
        if g.phase(v) == parameter() || g.phase(v) == target() {
            let p = g.add_vertex(VType::Z);
            
            if g.phase(v) == parameter() {
                param_verts.push(p);
            } else {
                match target_vert {
                    None => { target_vert = Some(p); }
                    Some(_) => { panic!("Var spider occurrs multiple times!"); }
                }
            }

            g.add_edge(v, p);
            if g.vertex_type(v) == VType::X {
                g.set_edge_type(v, p, EType::H);
            }

            // Remove phase from original spider
            g.set_phase(v, Rational::new(0, 1));
        }
    }
    let target_vert = match target_vert {
        None => { panic!("No var spider found!"); }
        Some(v) => v
    };

    // Apply to |0> state
    for _ in 0..circ.num_qubits() {
        g.plug_input(0, BasisElem::Z0);
    }

    // Construct expectation value diagram
    let g_adj = g.to_adjoint();
    let mut expval = g;
    let h = hamiltonian(pauli_hamiltonian);
    expval.plug(&h);
    let rename_map = plug_vmap(&mut expval, &g_adj);

    // Remember names of the parameter spiders
    let target_vert_2 = (target_vert, rename_map[&target_vert]); 
    let mut param_verts_2: Vec<(V,V)> = Vec::new();
    for v in param_verts {
        param_verts_2.push((v, rename_map[&v]));
    }

    // Add copy of expval diagram
    let expval_copy = expval.clone();
    let mut var = expval;
    let rename_map = plug_vmap(&mut var, &expval_copy);

    // Remember names of the parameter spiders
    let target_vert_4 = (target_vert_2.0, target_vert_2.1, rename_map[&target_vert_2.0], rename_map[&target_vert_2.1]); 
    let mut param_verts_4: Vec<(V,V,V,V)> = Vec::new();
    for (v1, v2) in param_verts_2 {
        param_verts_4.push((v1, v2, rename_map[&v1], rename_map[&v2]));
    }

    // Add cycles
    for (v1_l, v1_r, v2_l, v2_r) in param_verts_4 {
        let z_l = var.add_vertex(VType::Z);
        let z_r = var.add_vertex(VType::Z);
        let z_b = var.add_vertex(VType::Z);
        let x_l = var.add_vertex(VType::X);
        let x_r = var.add_vertex(VType::X);
        let x_t = var.add_vertex(VType::X);

        var.add_edge(v1_l, z_l);
        var.add_edge(v1_r, z_r);
        var.add_edge(v2_l, x_l);
        var.add_edge(v2_r, x_r);
        var.add_edge(z_l, x_l);
        var.add_edge(z_r, x_r);
        var.add_edge(z_l, x_t);
        var.add_edge(z_r, x_t);
        var.add_edge(x_l, z_b);
        var.add_edge(x_r, z_b);

        let tri = triangle();
        let vmap = var.append_graph(&tri);
        var.add_edge(x_t, vmap[&tri.inputs()[0]]);
        var.add_edge(z_b, vmap[&tri.outputs()[0]]);
        var.set_vertex_type(vmap[&tri.inputs()[0]], VType::Z);
        var.set_vertex_type(vmap[&tri.outputs()[0]], VType::Z);

        // Scalar 3*sqrt(2)
        var.scalar_mut().mul_sqrt2_pow(1);
        var.scalar_mut().mul_sqrt2_pow(1);
        var.scalar_mut().mul_sqrt2_pow(1);
    }

    // Add Var cycle
    let (v1_l, v1_r, v2_l, v2_r) = target_vert_4;
    let z_l = var.add_vertex(VType::Z);
    let z_r = var.add_vertex(VType::Z);
    let x_l = var.add_vertex_with_phase(VType::X, Rational::new(1, 1));
    let x_r = var.add_vertex_with_phase(VType::X, Rational::new(1, 1));
    let x_t = var.add_vertex_with_phase(VType::X, Rational::new(1, 1));
    var.add_edge(v1_l, z_l);
    var.add_edge(v1_r, z_r);
    var.add_edge(v2_l, x_l);
    var.add_edge(v2_r, x_r);
    var.add_edge(z_l, x_l);
    var.add_edge(z_r, x_r);
    var.add_edge(z_l, x_t);
    var.add_edge(z_r, x_t);

    return var;
}


pub fn select_target(circ: &mut Circuit, i: usize) {
    let mut j = 0;
    for k in 0..circ.num_gates() {
        if circ.gates[k].phase == parameter() {
            if j == i {
                circ.gates[k].phase = target();
                return;
            }
            j += 1;
        }
    }
}


pub fn compute_variance(circ: &mut Circuit, hamiltonian: &str) -> ScalarN {
    let mut var = create_diagram(&circ, hamiltonian);

    quizx::simplify::full_simp(&mut var);

    let mut d = quizx::decompose::Decomposer::new(&var);
    d.with_full_simp();
    d.use_cats(true);

    //let d = d.decomp_parallel(3);
    d.decomp_all();
    
    return d.scalar;
}
