#!/bin/sh

lib_path="$(realpath ../../../target/debug/)"
exec cc -O0 -g -o main "-L$lib_path" -xc main.c -rpath "$lib_path" -lsysinfo

