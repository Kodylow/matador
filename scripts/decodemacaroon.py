from pymacaroons import Macaroon
import base64
import argparse

def decode_and_inspect_macaroon(encoded_macaroon):
    """
    Decodes and inspects a given macaroon.

    Args:
        encoded_macaroon (str): The base64 encoded macaroon string.

    Returns:
        str: Human-readable representation of the macaroon.
    """

    # Decode the base64 encoded macaroon
    serialized_macaroon = base64.urlsafe_b64decode(encoded_macaroon)

    # Parse the serialized macaroon
    macaroon = Macaroon.deserialize(serialized_macaroon)

    # Extract and print the various components
    location = macaroon.location
    identifier = macaroon.identifier
    signature = macaroon.signature

    return f"Location: {location}\nIdentifier: {identifier}\nSignature: {signature}"

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Decode and inspect a macaroon.')
    parser.add_argument('macaroon', help='The base64 encoded macaroon string.')
    args = parser.parse_args()

    print(decode_and_inspect_macaroon(args.macaroon))