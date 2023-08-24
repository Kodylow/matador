import subprocess
import re
import sys

def curl_command(command: str) -> str:
    """Executes a curl command and returns its output."""
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    return result.stdout + result.stderr

def extract_token_invoice(header: str):
    """Extracts the token and invoice from the www-authenticate header."""
    match = re.search(r'L402 token="([^"]+)", invoice="([^"]+)"', header)
    if match:
        return match.group(1), match.group(2)
    else:
        raise ValueError("Failed to extract token and invoice")

def main():
    cmd = ' '.join(['curl'] + sys.argv[1:])
    
    output = curl_command(cmd)

    # Check if we received a L402 response
    if "HTTP/1.1 402 Payment Required" in output:
        www_authenticate = re.search(r'www-authenticate: (.+)', output)
        
        if www_authenticate:
            www_authenticate_header = www_authenticate.group(1)
            token, invoice = extract_token_invoice(www_authenticate_header)
            print(f"Token: {token}\nInvoice: {invoice}")
            
            # Get the preimage from the user
            preimage = input("Please enter the preimage after paying: ")
            
            # Print the Authorization header
            auth_header = f'"Authorization: L402 {token}:{preimage}"'
            print(f"Auth header to resend curl with: -H \n{auth_header}\n")
        else:
            print("Could not find www-authenticate header.")
    else:
        print(output)

if __name__ == '__main__':
    main()
