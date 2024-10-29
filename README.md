gemfeed2atom
============

This is a tool for reading a directory containing Gemini posts and generating an atom.xml feed

Example usage
-------------

    $ gemfeed2atom --base-url gemini://gemini.ucant.org/gemlog/ --feed-dir . --title "Ucant Gemlog" > atom.xml

Compiling
---------

    $ cargo build

Installation
------------

    $ cargo install --path .

