#!/bin/bash

cmd=$1

rm -f cmd_times.tmp

for i in $(seq 1 1 100)
do
    (time ${cmd}) 2>>cmd_times.tmp
done

cat cmd_times.tmp |\
    rg "^user\s+([0-9\.]+)m([0-9\.]+)s$" \
        --replace '$1,$2' > cmd_times.tmp.1
mv cmd_times.tmp.1 cmd_times.tmp
