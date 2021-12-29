#!/bin/bash

cargo build --release
cp ./target/release/i3ws ~/.local/bin
