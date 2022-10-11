#!/usr/bin/env python

"""This program shows `hyperfine` benchmark results as a box and whisker plot.

Quoting from the matplotlib documentation:
    The box extends from the lower to upper quartile values of the data, with
    a line at the median. The whiskers extend from the box to show the range
    of the data. Flier points are those past the end of the whiskers.
"""

import argparse
import json
import matplotlib.pyplot as plt
import pandas as pd

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file", help="JSON file with benchmark results")
parser.add_argument("-t","--title", help="Plot Title")
parser.add_argument(
    "-l","--labels", help="Comma-separated list of entries for the plot legend"
)
parser.add_argument(
	"-b","--benches", help="comma separated list of benchmark names"
)
parser.add_argument(
    "-o", "--output", help="Save image to the given filename."
)
parser.add_argument("-y","--y-label")
parser.add_argument("-x","--x-label")
parser.add_argument("-s","--sum",action="store_true",help="Sum the results of the fed-mrmr benchmarks (first two benches)")
parser.add_argument("--sum-label",help="Label for the sum of the fed-mrmr benchmarks")
parser.add_argument("--federated",action="store_true",help="Plot the federated for the fed-mrmr matrix generation(min of each node)")
parser.add_argument("--table",action="store_true",help="Print the table of the results")
parser.add_argument("--big",type=int,help="big fontsize",default=20)
parser.add_argument("--small",type=int,help="small fontsize",default=16)
parser.add_argument("--width",type=float,help="small fontsize",default=.8)

args = parser.parse_args()

with open(args.file) as f:
    results = json.load(f)["results"]

if args.labels:
    labels = args.labels.split(",")
else:
    labels = [b["command"] for b in results]


data = {}
benches = args.benches.split(",")
for i,bench in enumerate(benches):
	data[bench] = [round(result["mean"],2) for result in results[i::len(benches)]]

if args.federated:
    federated_max_matrix_bench =[]
    for partition in labels:
        with open(f"results/partitions/benchmark_federated_mnist_matrix_partition_{partition}.json") as f:
            partition_results = json.load(f)["results"]
            federated_max_matrix_bench.append(round(max([result["mean"] for result in partition_results]),2))
    bench_name = "max fed-mRMR matrix"
    benches.insert(1,bench_name)
    data[bench_name] = federated_max_matrix_bench
    if args.sum:
        data[args.sum_label] = [sum(x) for x in zip(data[benches[0]],data[benches[1]],data[benches[2]])]
        benches.insert(3,args.sum_label)

if args.sum and not args.federated:
        data[args.sum_label] = [sum(x) for x in zip(data[benches[0]],data[benches[1]])]
        benches.insert(2,args.sum_label)
        
df = pd.DataFrame(data, index=labels,columns=benches)

# Plot bar df with column labels as 2 decimal float
ax = df.plot.bar(rot=0,colormap="copper",width=args.width,stacked=args.federated)

plt.legend(fontsize=args.big)
plt.xticks(fontsize=args.small)
plt.yticks(fontsize=args.small)


# Axis labels
if args.title:
    ax.set_title(args.title,fontsize=args.big)

if args.y_label:
    ax.set_ylabel(args.y_label,fontsize=args.big)
else:
    ax.set_ylabel('Time (seconds)',fontsize=args.big)

if args.x_label:
    ax.set_xlabel(args.x_label,fontsize=args.big)

label_colors = ['white', 'black', 'black']
for i,c in enumerate(ax.containers):

    # Optional: if the segment is small or 0, customize the labels
    labels = [f"{v.get_height():.2f}" for v in c]
    
    # remove the labels parameter if it's not needed for customized labels
    ax.bar_label(c,color=label_colors[i], labels=labels, label_type='edge',fmt="%0.2f",fontsize=args.small,padding=0)



if args.table:
    print(f"\\begin{{tabular}}{{{'c' * (len(benches)+1)}}}")
    print(f"\hline")
    print(f"{' & '.join(['Dataset'] + benches)}\\\\")
    print(f"\hline")
    for label in labels:
        print(f"{' & '.join([label]+[f'{df[bench][label]:.2f}' for bench in benches])} \\\\")
    print(f"\hline")
    print(f"\end{{tabular}}")
else:
    if args.output:
        plt.savefig(args.output)
    else:
        plt.show()
        
# \begin{tabular}{ccc}
# \hline
# Datasets           & \# of features to select \\ \hline
# Lung               & 300                                 \\
# Colon              & 300                                \\
# Lymphoma           & 300                                \\
# Letter-recognition & 16                                 \\
# Connect-4          & 42                               \\
# MNIST              & 30                              \\
# \hline
# \end{tabular}