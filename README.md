# git-user

A simple cross-platform cli tool for switching between git profiles.

## Usage

The tool has a very straightforward terminal interface

```console
$ git-user

┌──────────┐
│ GIT-USER │
└──────────┘

Pick profile for this repo
  [1] Emploee 123 (employee123@corp.com) -> work profile
  [2] John Doe (johndoe@example.com) -> personal profile
  [3] Jane Doe (janedoe@example.com) -> another personal profile
  [a] Add a new profile
  [q] Quit

-> Option: 2

User John Doe (johndoe@example.com) has been selected for this repo
```

All profiles are stored in `~/.config/git-user.txt` in the following format

```
Emploee 123:employee123@corp.com    # work profile
John Doe:johndoe@example.com        # personal profile
Jane Doe:janedoe@example.com        # another personal profile
```

## License

MIT
