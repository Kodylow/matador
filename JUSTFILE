watch:
    cargo watch -q -c -w src/ -x run
test:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

models:
    curl -v http://localhost:8080/openai/v1/models && curl -v http://localhost:8080/openai/v1/models/text-davinci-003

models header:
    curl -v http://localhost:8080/openai/v1/models {{header}} && curl -v http://localhost:8080/openai/v1/models/text-davinci-003 {{header}}

chat:
    curl -v http://localhost:8080/openai/v1/chat/completions -H "Content-Type: application/json" -d '{"model": "gpt-3.5-turbo","messages": [{"role": "system","content": "You are a helpful assistant."},{"role": "user","content": "Hello!"}]}'

chat header:
    curl -v http://localhost:8080/openai/v1/chat/completions {{header}} -H "Content-Type: application/json" -d '{"model": "gpt-3.5-turbo","messages": [{"role": "system","content": "You are a helpful assistant."},{"role": "user","content": "Hello!"}]}'

image-edit:
    curl -v http://localhost:8080/openai/v1/images/edits -F image="@otter.png" -F prompt="A cute baby sea otter wearing a beret" -F n=2 -F size="1024x1024"

image-edit header:
    curl -v http://localhost:8080/openai/v1/images/edits {{header}} -F image="@otter.png" -F prompt="A cute baby sea otter wearing a beret" -F n=2 -F size="1024x1024"

image-create:
    curl -v http://localhost:8080/openai/v1/images/generations -H "Content-Type: application/json" -d '{"prompt": "A cute baby sea otter","n": 2,"size": "1024x1024"}'

image-create header:
    curl -v http://localhost:8080/openai/v1/images/generations {{header}} -H "Content-Type: application/json" -d '{"prompt": "A cute baby sea otter","n": 2,"size": "1024x1024"}'

image-vary:
    curl -v http://localhost:8080/openai/v1/images/variations -F image="@otter.png" -F n=2 -F size="1024x1024"

image-vary header:
    curl -v http://localhost:8080/openai/v1/images/variations {{header}} -F image="@otter.png" -F n=2 -F size="1024x1024"

embeddings:
    curl -v http://localhost:8080/openai/v1/embeddings -H "Content-Type: application/json" -d '{"input": "The food was delicious and the waiter...","model": "text-embedding-ada-002"}'

embeddings header:
    curl -v http://localhost:8080/openai/v1/embeddings {{header}} -H "Content-Type: application/json" -d '{"input": "The food was delicious and the waiter...","model": "text-embedding-ada-002"}'

makersuite-text-create:
    curl -v http://localhost:8080/makersuite/v1beta2/models/text-bison-001:generateText -H 'Content-Type: application/json' -X POST -d '{ "prompt": { \"text": "Write a story about a magic backpack."}}'

makersuite-text-create header:
    curl -v http://localhost:8080/makersuite/v1beta2/models/text-bison-001:generateText {{header}} -H 'Content-Type: application/json' -X POST -d '{ "prompt": { \"text": "Write a story about a magic backpack."}}'

makersuite-embed-text:
    curl -v http://localhost:8080/makersuite/v1beta2/models/embedding-gecko-001:embedText -H 'Content-Type: application/json' -X POST -d '{"text": "say something nice!"}'
    
makersuite-embed-text header:
    curl -v http://localhost:8080/makersuite/v1beta2/models/embedding-gecko-001:embedText {{header}} -H 'Content-Type: application/json' -X POST -d '{"text": "say something nice!"}'