#!/bin/bash

set -euo pipefail

for i in $(seq 1 24);
do
  cargo run --bin day$i
done
