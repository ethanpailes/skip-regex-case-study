#!/bin/sh

pushd scrape
cargo build --release
popd

echo "skip"
time scrape/target/release/scrape server.log

echo ""
echo "debug"
time scrape/target/release/scrape -d server.log

echo ""
echo "standard"
time scrape/target/release/scrape -s server.log
