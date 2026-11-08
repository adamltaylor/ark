# Replica's Ark

This is a fork of [posit-dev/ark](https://github.com/posit-dev/ark), the R
kernel, for [Replica](https://github.com/adamltaylor/Replica), which has it as
a submodule. It is upstream Ark at a release tag, with Replica's patches on
top as a short line of commits, one patch per commit.

## Branches

- `replica/<upstream version>`, e.g. `replica/0.1.252`: the patches on top of
  that upstream release tag. A branch is never rewritten once pushed, since
  Replica's history points its submodule at its commits; moving to a new
  upstream release makes a new branch.
- `main` and upstream's tags are upstream's. Don't push to this fork's `main`:
  upstream's workflows run on pushes to it (and would try to make upstream
  releases if Actions is enabled here). Upstream is fetched into local clones
  as the `upstream` remote instead.

Replica builds Ark from its submodule, with Cargo, as part of its own build,
and ships the binary beside its own; there are no releases of this fork.

## Moving to a new upstream release

In a clone with upstream as a remote
(`git remote add upstream ssh://git@github.com/posit-dev/ark.git`):

```sh
git config rerere.enabled true    # once per clone: reuse conflict resolutions
git fetch upstream --tags
old=0.1.252 new=0.1.260
git switch -c replica/$new replica/$old
git rebase --onto $new $old replica/$new
git range-diff $old..replica/$old $new..replica/$new   # each patch survived?
just test                                               # Ark's own tests
git push -u origin replica/$new
```

Then, in Replica: point the submodule at the new branch (`branch` in
`.gitmodules`) and its last commit, rebuild, and run Replica's tests, which
use the Ark it builds. Before moving, recheck the kernel protocol notes in
Replica's AGENTS.md, which describe this version of Ark.

## The patches

Each patch is one commit, its message starting `replica: `, with a test in
Ark's suite where it can have one. Each is listed here: what it does, why
Replica needs it, and its upstream issue or pull request if there is one.

This file is Replica's own, not a patch, and conflicts with nothing
upstream.

No patches yet.
