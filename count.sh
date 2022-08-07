#!/bin/sh

echo unsafe $(rg -IcU "unsafe[\s]*\{" *kyu | paste -sd+ | bc)
echo bytes $(rg -Ic "\.bytes\(\)|\.as_bytes\(\)|\.as_bytes_mut\(\)|\.as_mut_vec\(\)" *kyu | paste -sd+ | bc)
