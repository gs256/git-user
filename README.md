# git-user

A simple cross-platform cli tool for switching between git profiles.

## Usage

The tool has a very straightforward terminal interface

```console
$ git-user

No user configured for this repo

Which profile to add? (from /home/username/.git-user.txt)
1. User1 - user1@example.com
2. User2 - user2@example.com
3. User3 - user3@example.com
Option: 2

User 'User2:user2@example.com' successfully configured
```

All profiles are stored in `~/.git-user.txt`

```
User1:user1@example.com
User2:user2@example.com
User3:user3@example.com
```

## License

MIT
