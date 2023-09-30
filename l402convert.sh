#!/bin/bash

# Check the number of arguments
if [[ $# -ne 2 ]]; then
    echo "Usage: $0 token preimage"
    exit 1
fi

token="$1"
preimage="$2"

# Print the result
echo -e "\n-H 'Authorization: L402 $token:$preimage'"
