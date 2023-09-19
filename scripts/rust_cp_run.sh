#!/bin/bash

# Check if Cargo.toml exists in the current directory
if [ ! -f Cargo.toml ]; then
    echo "Error: Cargo.toml not found in the current directory."
    exit 1
fi

# Define input and output file paths
input_file="inp.txt"
output_file="out.txt"
error_file="err.txt"  # Add a new variable for error output

# Check if input file exists
if [ ! -f "$input_file" ]; then
    echo "Error: $input_file not found."
    exit 1
fi

# Record the start time
start_time=$(date +%s.%N)

# Build and run the Rust program using Cargo, pipe input from input.txt, and save the output and errors to respective files
cargo run --package rust_prac --bin rust_prac < "$input_file" > "$output_file" 2> "$error_file"

# Capture the exit status of the Rust program
rust_exit_status=$?

# Calculate the elapsed time
end_time=$(date +%s.%N)
elapsed_time=$(echo "$end_time - $start_time" | bc)

# Check if the Rust program ran successfully
if [ $rust_exit_status -eq 0 ]; then
    echo "Rust program executed successfully. Output saved to $output_file."
    echo "Time taken: $elapsed_time seconds"
else
    echo "Error: Rust program execution failed. Errors saved to $error_file."
    echo "Time taken: $elapsed_time seconds"
fi
