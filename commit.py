# https://www.steadylearner.com/blog/read/How-to-automatically-commit-files-to-GitHub-with-Python

import subprocess as cmd

cp = cmd.run("git add .", check=True, shell=True)

default = "Edit README.md"
response = input("Do you want to use the default message for this commit?([y]/n)\n")

if response.startswith('n'):
    message = input("What message you want?\n")
    if not message:
        message = default
else:
    message = default

cp = cmd.run(f"git commit -m '{message}'", check=True, shell=True)
cp = cmd.run("git push -u origin master -f", check=True, shell=True)