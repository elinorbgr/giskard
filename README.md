# Giskard: the todo.txt butler

Giskard is a toolkit for using todo.txt files

As a library, it provides an abstraction framework to manipulate `todo.txt` and `done.txt` files.

It is also a CLI for viewing, editing, searching, and generaly using your `todo.txt` files.

## CLI usage

You need to build the cargo project with the `cli` feature flag for it to be built.

Giskard is configured via a config file in `$XDG_CONFIG_HOME/giskard/config.toml`, its format is:

```toml
# An array of task files, giskard can work with multiple of them
[[taskfiles]]
# A name given to this taskfile
name = "Default"
# The path of this taskfile
task_file = "/home/levans/Nextcloud/todo.txt"
# An optional path to the associated "done" file, to archive finished tasks
done_file = "/home/levans/Nextcloud/done.txt"
# If no "done" file is specified, whether the done tasks should be kept in the
# the taskfile, or discarded.
discard_done = false
```

## Status

This is very much WIP, and currently the giskard is very feature lacking. Any help is welcome if you are interested.
