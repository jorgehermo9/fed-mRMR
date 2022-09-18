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
import numpy as np

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file", help="JSON file with benchmark results")
parser.add_argument("--title", help="Plot Title")
parser.add_argument(
    "--labels", help="Comma-separated list of entries for the plot legend"
)
parser.add_argument(
    "-o", "--output", help="Save image to the given filename."
)
parser.add_argument("-f","--first")
parser.add_argument("-s","--second")
parser.add_argument("-y","--y-label")
parser.add_argument("-x","--x-label")


args = parser.parse_args()

with open(args.file) as f:
    results = json.load(f)["results"]

if args.labels:
    labels = args.labels.split(",")
else:
    labels = [b["command"] for b in results]

first_pair_label = args.first
second_pair_label = args.second

first_pair_times = [round(b["mean"],3) for b in results[::2]]
second_pair_times = [round(b["mean"],3) for b in results[1::2]]


x = np.arange(len(labels))  # the label locations
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(x - width/2, first_pair_times, width, label=first_pair_label,color="dimgrey")
rects2 = ax.bar(x + width/2, second_pair_times, width, label=second_pair_label,color="lightgrey")


if args.y_label:
    ax.set_ylabel(args.y_label)
else:
    ax.set_ylabel('Time (seconds)')

if args.x_label:
    ax.set_xlabel(args.x_label)


if args.title: 
    ax.set_title(args.title)

ax.set_xticks(x, labels)
ax.legend()

ax.bar_label(rects1, padding=3)
ax.bar_label(rects2, padding=3)

fig.tight_layout()

if args.output:
    plt.savefig(args.output)
else:
    plt.show()
