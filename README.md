# Numerical Barren Plateau Detection

Code to reproduce the numerical results and graphs from my [MSc thesis](https://arxiv.org/abs/2210.11523).


## Building

* Install Rust from https://www.rust-lang.org/tools/install
* Build the tool using `cargo build --release`

## Usage

The binary will be availabe at `target/release/bpdetect` and accepts the following arguments: `bpdetect circuitName numQubits numLayers pauliString parameterIdx`:

* `circuitName` must be one of: `introExample`, `iqpExample`, `sim1`, `sim2`, `sim9`, `sim10`, `sim11`, `sim12`, `sim15`, `iqp1`, `iqp2`, `iqp3`. Here, `introExample` and `iqpExample` are the example circuits we discuss in Sections 5.4.1 and 5.5.4 respectively.

* `pauliString` represents the measurement Hamiltonian, for example `ZXIIYX`. Should have length `numQubits`.

* `parameterIdx` is the parameter with regards to which the derivative is analysed. Counting starts at 0.


## Reproducing Results

To reproduce the numerical results from the thesis, first run `python3 experiments.py`.
This will perform the variance computation using a single CPU core and take about 2 hours to run depending on hardware.
The execution time can be greatly sped up by utilising multiples CPU cores.
For example, `python3 experiments.py 10` will use 10 cores and takes about 12 minutes to run on my machine.

To produce the graphs, run `python3 plot.py` afterwards.
Note that this requires `pdflatex` to be in the system path.

