# Janet

**GitLab Companion Bot** _(work in progress)_

# Supported commands

TODO

# Installation

TODO

# Contributing

## Tests

Janet uses two types of tests:

- "fast but not necessarily determinate" ones (folks like to call them `unit tests`, although they aren't strictly
  _unit_ in here)
- "slow but thorough" ones (so like `end-to-end` tests).

You can run `fast` tests using standard:

```shell
$ cargo test --all
```

... and, as for the `slow` ones (friendly warning: they are somewhat work-in-progress at the moment):

```shell
$ cd libs/system
$ cargo test --features e2e -- --test-threads=1
```

# License

Copyright (c) 2020, Patryk Wychowaniec wychowaniec.patryk@gmail.com.

Licensed under the MIT license.