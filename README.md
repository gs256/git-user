# git-user

A simple cross-platform cli tool for switching between git profiles.

## Usage

The tool has a very straightforward terminal interface

```console
$ git-user

No user configured for this repo

Which profile to use in this repo? (from /home/username/.git-user.txt)
1. Profile 'user1:user1@example.com' work profile
2. Profile 'user2:user2@example.com' personal profile
3. Profile 'user3:user3@example.com' another personal profile
4. Add a new profile

Option: 2

User 'user2:user2@example.com' successfully configured
```

All profiles are stored in `~/.git-user.txt`

```
user1:user1@example.com    # work profile
user2:user2@example.com    # personal profile
user3:user3@example.com    # another personal profile
```

## License

MIT
