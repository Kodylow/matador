#!/bin/bash

make_api_call() {
  local url=$1
  local data=$2
  local headers=$3
  local method=$4
  local output_file=$5

  if [[ "$headers" == "Content-Type: multipart/form-data" ]]; then
    if [ -z "$output_file" ]
    then
      response=$(curl -s -o /dev/null -w "%{http_code}" -H "$headers" -F "$data" -X "$method" "$url")
      echo "Response: $response"
    else
      response=$(curl -s -o "$output_file" -w "%{http_code}" -H "$headers" -F "$data" -X "$method" "$url")
      echo "Response: $response"
      echo "Response saved to $output_file"
    fi
  else
    if [ -z "$output_file" ]
    then
      response=$(curl -s -o /dev/null -w "%{http_code}" -H "$headers" -d "$data" -X "$method" "$url")
      echo "Response: $response"
    else
      response=$(curl -s -o "$output_file" -w "%{http_code}" -H "$headers" -d "$data" -X "$method" "$url")
      echo "Response: $response"
      echo "Response saved to $output_file"
    fi
  fi
}

test=$1

if [[ -z "$test" || "$test" == "openai" ]]; then
  # OpenAI
  make_api_call "http://localhost:8080/openai/v1/chat/completions" '{
      "model": "gpt-3.5-turbo",
      "messages": [
        {
          "role": "system",
          "content": "You are a helpful assistant."
        },
        {
          "role": "user",
          "content": "Hello!"
        }
      ]
    }' "Content-Type: application/json" "POST"
fi

if [[ -z "$test" || "$test" == "palm" ]]; then
  # Palm
  make_api_call "http://localhost:8080/palm/v1beta2/models/text-bison-001:generateText" '{
          "prompt": {
                "text": "Write a story about satoshi nakamoto."
                }
            }' "Content-Type: application/json" "POST"
fi

if [[ -z "$test" || "$test" == "replicate" ]]; then
  # Replicate
  make_api_call "http://localhost:8080/replicate/v1/predictions" '{"version": "5c7d5dc6dd8bf75c1acaa8565735e7986bc5b66206b55cca93cb72c9bf15ccaa", "input": {"text": "Alice"}}' "Content-Type: application/json" "POST"
fi

if [[ -z "$test" || "$test" == "clipdrop" ]]; then
  # Clipdrop
  timestamp=$(date +%Y%m%d%H%M%S)
  make_api_call "http://localhost:8080/clipdrop/text-to-image/v1" 'prompt=shot of satoshi nakamoto programming bitcoin' "Content-Type: multipart/form-data" "POST" "clipdrop_${timestamp}.png"
fi

if [[ -z "$test" || "$test" == "anthropic" ]]; then
  # Anthropic
  make_api_call "http://localhost:8080/anthropic/v1/complete" '
  {
    "model": "claude-2",
    "prompt": "\n\nHuman: Hello, world!\n\nAssistant:",
    "max_tokens_to_sample": 256
  }' "Content-Type: application/json" "POST"
fi