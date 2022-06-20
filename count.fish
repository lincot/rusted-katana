#!/usr/bin/env fish

echo unsafe (rg -g '!/README.md' "unsafe \{" | wc -l)
echo bytes (rg -g '!/README.md' "\.bytes\(\)|\.as_bytes\(\)|\.as_mut_vec\(\)" | wc -l)
