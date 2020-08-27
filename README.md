# Janet

**GitLab Companion Bot** _(work in progress)_

Janet is an interactive self-hosted bot enhancing experience of vanilla's GitLab.

![janet.gif](https://media1.giphy.com/media/xUOxeRRkTYdQJfyy2Y/giphy.gif)

# Features

## Merge request dependencies

While going through a merge request, you can drop a comment saying e.g. `@janet depends on other-project!123` and Janet
will let you know when that merge request gets merged, closed or re-opened.

> Due diligence: **GitLab Premium** offers [a similar feature](https://docs.gitlab.com/ee/user/project/merge_requests/merge_request_dependencies.html) too.

## Reminders

While going through a merge request, you can drop a comment saying e.g. `@janet remind me tomorrow`,
`@janet remind me in 3d 2h` and Janet will ping you after that time passes.

# Installation

TODO

# Roadmap

Before official release, I'd like to:

- Implement support for reminders (e.g. `@janet remind me next week`),
- Implement support for custom commands (e.g. when comment matches a predefined regular expression, Janet will run given
  script in the background).

# Contributing

All merge requests, feature ideas & bug reports are welcome!

## Example workflows

### Implementing a new command

Janet's human-to-bot interface (so like _comment parser_) has been implemented inside the `libs/interface` crate, with
the two most important types being `Command` and `Event` - if you want to implement a new command, that's the right
place to start.

Eventually all commands and events are handled inside the `libs/system` crate; if you want to get inspired, take a look
at `libs/system/src/tasks/handle_commands/merge_request/hi.rs`, which is the simplest command supported by Janet.

Finally, end-to-end tests are implemented inside the `tests` directory - don't forget to add a few :-)

## Tests

To launch all (unit & integration) tests, use good-old:

```shell
$ cargo test --workspace
```

_(integration tests require around a megabyte of free space inside the `/tmp` directory.)_

If you have Nix, you can use `nix-build .` as a shorthand, because it automatically launches all the tests too.

## Formatting

```shell
$ cargo fmt
```

# License

Copyright (c) 2020, Patryk Wychowaniec, <wychowaniec.patryk@gmail.com>

Licensed under the MIT license.