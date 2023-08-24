import sys

def get_token_from_header(header):
    if not header.startswith('www-authenticate: L402 token='):
        raise ValueError("Invalid header format.")
    token = header.split('token="')[1].split('"')[0]
    return token

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: script_name www_authenticate_header preimage")
        sys.exit(1)

    preimage = sys.argv[1]
    www_authenticate_header = sys.argv[2]

    token = get_token_from_header(www_authenticate_header)
    output = f"\n-H 'Authorization: L402 {token}:{preimage}'"
    print(output)
