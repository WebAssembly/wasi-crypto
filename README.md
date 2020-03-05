# WASI Cryptography APIs

This repository is for development of Cryptography API proposals for the
[WASI Subgroup] of the [WebAssembly Community Group]. Please refer to those
groups' documentation for more information on their processes, goals, scope,
and deliverables.

[WASI Subgroup]: https://github.com/WebAssembly/WASI
[WebAssembly Community Group]: https://www.w3.org/community/webassembly/

#### Synchronizing with the WASI Subgroup repository

The `phases` and `tools` directories are periodically sync'ed with the
[WASI Subgroup] repository, with the following command:

```sh
$ git remote add WASI https://github.com/WebAssembly/WASI.git
$ git fetch WASI
$ git merge -s ours --allow-unrelated-histories --no-commit WASI/master
$ git read-tree --prefix=phases -u WASI/master:phases
$ git read-tree --prefix=tools -u WASI/master:tools
$ git commit
```
