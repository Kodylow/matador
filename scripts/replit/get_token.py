from replit_identity_token_manager import ReplitIdentityTokenManager

token_manager = ReplitIdentityTokenManager()
token = token_manager.get_token()
print(token)