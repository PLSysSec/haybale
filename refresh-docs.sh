#!/bin/zsh

# Requires ghp-import to be installed (e.g. via pip3)

rm -rf target/doc &&  # purge old docs that may include docs for deps
cargo doc --no-deps --features=llvm-13 &&  # document just this crate
echo "<meta http-equiv=refresh content=0;url=haybale/index.html>" > target/doc/index.html &&  # put in the top-level redirect
ghp-import -np target/doc &&  # publish to gh-pages branch
rm -rf target/doc &&  # kill the docs that were just this crate
cargo doc --features=llvm-13  # regenerate all docs (including deps) for local use
