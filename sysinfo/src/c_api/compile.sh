#!/bin/sh

lib_path="$(realpath ../../../target/debug/)"
exec cc -o main "-L$lib_path" -xc main.c -rpath "$lib_path" -lsysinfo

