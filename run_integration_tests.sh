#!/bin/bash
./build.sh
cp tests/test_maestro.py target/release/test_maestro.py
cp tests/expected.json target/release/expected.json
python target/release/test_maestro.py