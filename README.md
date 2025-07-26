![tests status](https://github.com/doomed-neko/tmapi/workflows/Rust/badge.svg)

# TempMailCLI

This is a command line application using my [tmapi](https://github.com/doomed-neko/tmapi/) library.
You can use it to read/remove emails easily!

```
Usage: tmcli [OPTIONS] <COMMAND>

Commands:
  list        List incoming emails
  open        Open a specific email
  delete-all  Delete all emails
  delete      Delete a specific email
  help        Print this message or the help of the given subcommand(s)

Options:
  -j, --json           Use json output
  -c, --color <COLOR>  Specify when to use colors [default: auto] [possible values: always, auto, never]
  -h, --help           Print help
  -V, --version        Print version
```
