# good-fences-rs

Written against `rustc 1.47.0 (18bf6b4f0 2020-10-07)`, w/ `neon-cli 0.5.1`

a rust reimplementation of [`smikula/good-fences`](https://github.com/smikula/good-fences), backed by [`good-fences-rs-core`](https://github.com/Adjective-Object/good-fences-rs-core).

## dev

to build this, you will need native headers for nodejs

```sh
dnf install nodejs-devel # get native dependencies
yarn # install js dependencies, download cargo dependencies, and build
yarn jest # run tests
```
