#!/usr/bin/env sh

svd2rust -i svd/LPC11xx-v6-z0.xml && rm -rf src && form -i lib.rs -o src/ && rm lib.rs && sed -i '1i#![allow(clippy::all)]' src/lib.rs && cargo fmt
