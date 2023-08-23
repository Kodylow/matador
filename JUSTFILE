watch:
    cargo watch -q -c -w src/ -x run
test:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

models:
    curl -v http://localhost:8080/openai/v1/models && curl -v http://localhost:8080/openai/v1/models/text-davinci-003

chat:
    curl -v http://localhost:8080/openai/v1/chat/completions -H "Content-Type: application/json" -d '{"model": "gpt-3.5-turbo","messages": [{"role": "system","content": "You are a helpful assistant."},{"role": "user","content": "Hello!"}]}'

image-edit:
    curl -v http://localhost:8080/openai/v1/images/edits -F image="@otter.png" -F prompt="A cute baby sea otter wearing a beret" -F n=2 -F size="1024x1024"

image-create:
    curl -v http://localhost:8080/openai/v1/images/generations -H "Content-Type: application/json" -d '{"prompt": "A cute baby sea otter","n": 2,"size": "1024x1024"}'

image-vary:
    curl -v http://localhost:8080/openai/v1/images/variations -F image="@otter.png" -F n=2 -F size="1024x1024"

embeddings:
    curl -v http://localhost:8080/openai/v1/embeddings -H "Authorization: Bearer $OPENAI_API_KEY" -H "Content-Type: application/json" -d '{"input": "The food was delicious and the waiter...","model": "text-embedding-ada-002"}'
