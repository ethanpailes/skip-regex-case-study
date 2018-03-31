#!/usr/bin/env python3
#
# Author: Ethan Pailes
#
# A little script to run a set of commands several time
# and produce a box-and-wiskers diagram of those commands.
#

import matplotlib.pyplot as plt; plt.rcdefaults()
import numpy as np
import matplotlib.pyplot as plt

import sys
import subprocess
import os
import pdb

args = {}
def main():
    global args
    args["bench_name"] = sys.argv[1]
    args["regen"] = sys.argv[2] == "yes"
    args["commands"] = []
    st = "name"
    name = None
    for a in sys.argv[3:]:
        if st == "name":
            name = a
            st = "cmd"
        elif st == "cmd":
            args["commands"].append((name, a))
            st = "name"

    if not args["regen"]:
        exec()

    graphit()

def exec():
    for (name, cmd) in args["commands"]:
        subprocess.call(["./gen_command_times.sh", cmd])
        os.rename("cmd_times.tmp", cmd_file_name(name))

def graphit():
    x = [list(slurp(name)) for (name, _) in args["commands"]]
    labels = [name for (name, _) in args["commands"]]
    [old, new] = [sum(d)/float(len(d)) for d in x]
    avg_speedup = old - new
    percent_change = - (((new - old)/old) * 100.0)

    plt.boxplot(
        x,
        labels=labels
    )
    plt.ylabel("User Time in Seconds")
    plt.title("{0:.3f} ({1:.2f} %) second average speedup"
                .format(avg_speedup, percent_change))
    plt.savefig(args["bench_name"] + ".png")

def slurp(cmd_name):
    with open(cmd_file_name(cmd_name)) as f:
        for line in f:
            mins, secs = line[:-1].split(",")
            mins, secs = float(mins), float(secs)
            yield (mins * 60.0) + secs

def cmd_file_name(cmd_name):
    return args["bench_name"] + "-" + cmd_name + ".times"

if __name__ == "__main__":
    main()

