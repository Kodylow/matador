watch: 
    cargo watch -q -c -w src/ -x run

test test_name:
    source ./test.sh {{test_name}}