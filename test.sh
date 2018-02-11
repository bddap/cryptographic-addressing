#!/bin/bash

cargo run --bin server | xargs -I{} cargo run --bin client {}
