#!/bin/sh

echo unsafe $(rg "unsafe \{" *kyu | wc -l)
echo bytes $(rg "\.bytes\(\)|\.as_bytes\(\)|\.as_bytes_mut\(\)|\.as_mut_vec\(\)" *kyu | wc -l)
