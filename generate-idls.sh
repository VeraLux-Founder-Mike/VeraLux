#!/bin/bash

# Create the destination directory if it doesn't exist
mkdir -p sdk/idl

# Copy the files to the destination directory
cp target/idl/veralux.json sdk/idl/
cp target/types/veralux.ts sdk/idl/

echo "Files copied successfully to sdk/idl/"