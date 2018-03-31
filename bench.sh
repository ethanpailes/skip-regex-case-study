#!/bin/bash

if [ -z ${1+x} ] ; then
    REGEN=no
else
    REGEN=$1
fi

RESOURCES_DIR=$HOME/repos/thesis/resources

./box_plot.py append ${REGEN} \
    standard "./scrape/target/release/scrape -a scrape/server.log" \
    validate-skip "./scrape/target/release/scrape -a -v scrape/server.log"

./box_plot.py append-just-match ${REGEN} \
    standard "./scrape/target/release/scrape -a scrape/appends.log" \
    validate-skip "./scrape/target/release/scrape -a -v scrape/appends.log"

./box_plot.py named ${REGEN} \
    standard "./scrape/target/release/scrape -n scrape/server.log" \
    validate-skip "./scrape/target/release/scrape -n -v scrape/server.log"

./box_plot.py append-named ${REGEN} \
    standard "./scrape/target/release/scrape -a -n scrape/server.log" \
    validate-skip "./scrape/target/release/scrape -a -n -v scrape/server.log"

cp *.png ${RESOURCES_DIR}
