import matplotlib.pyplot as plt
import numpy as np


data = {}

def parse(v):
    if v.isspace():
        return None, None
    try:
        return (int(v), int(v))
    except:
        a, b = v.split('*')
        _, a = a.split('^')
        b = b[2:-2]
        return (2**(int(a)) * int(b), (int(a), int(b)))
    

with open('result.txt') as f:
    for line in f.readlines():
        if line.isspace():
            continue
        d, var = line.split(':')
        s = d.split('-')
        if len(s) == 4:
            ansatz, n, l, h = s
            p = 0
        else:
            ansatz, n, l, h, p = s
        if ansatz not in data:
            data[ansatz] = []
        var, v = parse(var)
        data[ansatz].append((int(n), int(l), h, int(p), var, v))


plt.rcParams.update({
    "text.usetex": True,
    "font.family": "serif",
    "font.serif": ["Computer Modern"],
    "font.size": 8,
    "pgf.texsystem": "pdflatex"
})

sim_ansatze = ["sim1", "sim2", "sim9", "sim10", "sim11", "sim12", "sim15"]
iqp_ansatze = ["iqp1", "iqp2", "iqp3"]


mpl_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']
colors_sim = { ansatz: c for (ansatz, c) in zip(sim_ansatze, mpl_colors) }
colors_iqp = { ansatz: c for (ansatz, c) in zip(iqp_ansatze, mpl_colors) }
colors = {**colors_sim, **colors_iqp}

def plot_variance(ansatze, selector, title, max_qubits, path, figsize=(5.5,2.5), by_layers=False, 
                  ticks=1, legend=True, ylabel="", xlabel="", legend_label="sim", log_plot=True,
                 legend_pos="lower left", legend_path=None):
    def plot_single_ansatz(ansatz):
        points = [(n, v) if not by_layers else (l,v) 
                  for (n,l,h,p,v,_) in data[ansatz] if selector(n,l,h,p) and (n if not by_layers else l) <= max_qubits]
        points = [(n, v) for (n, v) in points if v != None and v != 0]
        points.sort()
        if len(points) == 0:
            return
        label = legend_label + r"$_{" + ansatz[3:] + "}$"
        plt.plot([x for (x, _) in points], [y for (_, y) in points], marker='.', label=label, c=colors[ansatz])
        
    fig, ax = plt.subplots(figsize=figsize)
    
    for ansatz in ansatze:
        plot_single_ansatz(ansatz)

    if legend:
        legend = plt.legend(loc=legend_pos, shadow=False, fancybox=False, framealpha=1, facecolor='white',
                            edgecolor='black')

    ax.xaxis.set_ticks(np.arange(2 if not by_layers else 1, max_qubits+1, ticks))

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)
    plt.title(title)

    ax.grid(True, alpha=0.2)
    if log_plot:
        ax.set_yscale('log', base=2)

    plt.tight_layout()
    fig.savefig(path)
    
    if legend_path is not None:
        figlegend = plt.figure(figsize=(2,2.5))
        figlegend.legend(ax.get_legend_handles_labels()[0], ax.get_legend_handles_labels()[1], loc='center',
                         shadow=False, fancybox=False, framealpha=1, facecolor='white',edgecolor='black')
        figlegend.tight_layout()
        figlegend.savefig(legend_path)
    

plot_variance(sim_ansatze,
              legend_label="Sim",
              max_qubits=25,
              ticks=2,
              selector=lambda n, l, h, p: l == 1 and h == "Z" and p == 0, 
              title="",
              xlabel="Number of qubits $n$",
              ylabel=r"Var$\left(\frac{\partial\langle H\rangle}{\partial\theta_1}\right)$",
              path='sim-singlelayer-p0.pdf',
              legend_path='sim-legend.pdf')

for i in range(1,10):
    plot_variance(sim_ansatze,
                  figsize=(2,2.5),
                  ticks=4,
                  legend=False,
                  max_qubits=25,
                  selector=lambda n, l, h, p: l == 1 and h == "Z" and p == i, 
                  title=r"Variance for $\theta_{" + str(i+1) + "}$",
                  xlabel="$n$",
                  path=f'sim-singlelayer-p{i}.pdf')

plot_variance(iqp_ansatze,
              legend_label="IQP",
              figsize=(4,2),
              max_qubits=25,
              ticks=2,
              selector=lambda n, l, h, p: l == 1, 
              title="",
              xlabel="Number of qubits $n$",
              ylabel=r"Var$\left(\frac{\partial\langle H\rangle}{\partial\theta_1}\right)$",
              path='iqp-singlelayer.pdf')

for i in range(2,5):
    plot_variance(iqp_ansatze,
                  legend_label="IQP",
                  log_plot=False,
                  max_qubits=25,
                  figsize=(2,1.5),
                  ticks=4,
                  selector=lambda n, l, h, p: l == i,
                  title=r"$\ell =" + str(i) + "$",
                  xlabel="$n$",
                  ylabel=r"Var$\left(\frac{\partial\langle H\rangle}{\partial\theta_1}\right)$" if i == 2 else "",
                  path=f'iqp1-layer{i}.pdf')
    
plot_variance(iqp_ansatze,
              legend_label="IQP",
              figsize=(5.5,2),
              legend_pos='lower right',
              by_layers=True,
              log_plot=False,
              max_qubits=20,
              ticks=2,
              selector=lambda n, l, h, p: n == 3, 
              title="$n = 3$",
              xlabel="Number of layers $\ell$",
              ylabel=r"Var$\left(\frac{\partial\langle H\rangle}{\partial\theta_1}\right)$",
              path='iqp1-layers.pdf')
