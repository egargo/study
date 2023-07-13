#!/usr/bin/env bash

mv -v rustlings rustlings.old;

git clone git@github.com:rust-lang/rustlings.git

cp -TRv rustlings.old rustlings

echo "Do you want to delete rustlings.old directory? [yN]: "
read option

if [ "$option" == "y" ]; then
    echo "Deleting the rustlings.old directory."
    rm -rf rustlings.old
else
    echo "Keeping rustlings.old directory."
fi

echo "Done!"
