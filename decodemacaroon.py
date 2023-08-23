from pymacaroons import Macaroon
import base64

def decode_and_inspect_macaroon(encoded_macaroon):
    """
    Decodes and inspects a given macaroon.

    Args:
        encoded_macaroon (str): The base64 encoded macaroon string.

    Returns:
        str: Human-readable representation of the macaroon.

    Example:
        encoded_macaroon = "Your encoded macaroon here..."
        print(decode_and_inspect_macaroon(encoded_macaroon))
    """

    # Add missing padding if necessary

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
    encoded_macaroon = "AgEIbG9jYXRpb24CAmlkAAJPcGF5bWVudF9oYXNoID0gM2IyMWQ2ZjlhMTMyYWZlODBhNDc4Njg0YTcxZmExY2Y3YmM3YWRhMTY2M2E2MGU4MzU5NjMxYjliZmM0ODA0OAAABiCqeicfNuvHbQlk9nvaf8bPfGICDqJHkv0lvEG40kPlnQ=="
    print(decode_and_inspect_macaroon(encoded_macaroon))
