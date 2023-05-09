#!/usr/bin/sh

mv -v rustlings rustlings.bak;

git rm -rf rustlings

curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash;

cp -vr rustlings.bak/exercises/ rustlings/exercises;

rm -rf rustlings/.git/;

git add rustlings;

echo "Done!"
