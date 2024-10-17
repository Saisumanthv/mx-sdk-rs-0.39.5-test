#!/bin/bash

cargo install drt-sc-meta

TARGET_DIR=$PWD/target

sc-meta all abi --path ./contracts
