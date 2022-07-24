# repomanager

A CLI tool for managing your repositories.

Features:
- create a repo
- delete a repo
___

## ***Prerequisites***

The tool expects a a file at `~/.repomanager/.github_info.txt`. The first line should be your github username, the second line should be your github [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)

Your personal access token needs to have permission to delete repositories.

##### Optional
Move into your binary path to call from anywhere 
```
sudo cp repomanager /bin/repomanager
```


___

#### Create a repo

##### Public Repo
```
repomanager create <repo name>
```
##### Private Repo
```
repomanager create <repo name> --private
```

#### Delete a repo
```
repomanager delete <repo name>
```

