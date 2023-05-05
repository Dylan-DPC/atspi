#!/bin/bash

cd atspi-codegen
cargo build && \
cd .. && \
./atspi-codegen/target/debug/gen_identify -f
./atspi-codegen/target/debug/gen_wai_events
