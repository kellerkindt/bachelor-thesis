#!/bin/bash

awk '{ total += $0 } END { print NR" "total/NR }' $1
