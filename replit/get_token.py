from replit_identity_token_manager import ReplitIdentityTokenManager
import json

token_manager = ReplitIdentityTokenManager()
token = token_manager.get_token()
timeout = token_manager.get_token_timeout()

print(json.dumps({
    "token": token,
    "timeout": timeout,
}))
