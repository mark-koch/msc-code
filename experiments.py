from subprocess import Popen, PIPE
import time
import sys

binary_path = 'target/release/bpdetect'
ansatze = ['sim1', 'sim2', 'sim9', 'sim10', 'sim11', 'sim12', 'sim15']

num_cores = int(sys.argv[1]) if len(sys.argv) > 1 else 1


def jobs():
    for n in range(2,26):
        for a in ansatze:
            yield {'ansatz': a, 'qubits': n, 'layers': 1, 'hamiltonian': 'Z'*n, 'parameter': 0}

    for n in range(2,26):
        for a in ansatze:
            for p in range(1, 10):
                yield {'ansatz': a, 'qubits': n, 'layers': 1, 'hamiltonian': 'Z'*n, 'parameter': p}
        
    for n in range(2,26):
        yield {'ansatz': 'iqp1', 'qubits': n, 'layers': 1, 'hamiltonian': 'Z'*n, 'parameter': 0}
        if n % 2 == 0:
            yield {'ansatz': 'iqp2', 'qubits': n, 'layers': 1, 'hamiltonian': 'YX'*(n//2), 'parameter': 0}
        yield {'ansatz': 'iqp3', 'qubits': n, 'layers': 1, 'hamiltonian': 'YX'*(n//2) + "Y"*(n % 2), 'parameter': 0}

    for l in [2, 3, 4]:
        for n in range(2,26):
            yield {'ansatz': 'iqp1', 'qubits': n, 'layers': l, 'hamiltonian': 'Z'*n, 'parameter': 0}

    for l in range(2,26):
        yield {'ansatz': 'iqp1', 'qubits': 3, 'layers': l, 'hamiltonian': 'Z'*3, 'parameter': 0}
    

jobs = jobs()
running = []

f = open('result.txt', 'w')

jobs_remaining = True

while jobs_remaining or len(running) > 0:
    # Start new jobs
    while jobs_remaining and len(running) < num_cores:
        try:
            job = next(jobs)
        except:
            jobs_remaining = False
            break
        p = Popen([binary_path, job['ansatz'], str(job['qubits']), str(job['layers']), job['hamiltonian'], str(job['parameter'])], stdout=PIPE, stderr=PIPE)
        running.append((job, p))

    time.sleep(.01)
    
    # Check if anyone is done
    terminated = []
    for (job, proc) in running:
        retcode = proc.poll()
        if retcode is not None:
            variance, _ = proc.communicate()
            variance = variance.decode('ASCII').rstrip()
            terminated.append((job, proc))
            s = f"{job['ansatz']}-{job['qubits']}-{job['layers']}-{job['hamiltonian'][0]}-{job['parameter']}: {variance}"
            f.write(s + "\n")
            print(s)

    for t in terminated:
        running.remove(t)

