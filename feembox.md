feembox(1) -- What if a feed, but it's a mailbox?
=================================================

## SYNOPSIS

`feembox` [-v] [MAILDIR] [FEED]<br />
`feembox` [-v] [MAILDIR] < feed.xml

## DESCRIPTION

`feembox` represents an (RSS/Atom/JSON) feed as a mailbox in the [maildir](https://cr.yp.to/proto/maildir.html) format.

## OPTIONS

  -v --verbose

    Print what's happening to the standard output

  [MAILDIR]

    Deliver to the specified directory instead of the CWD

    Parents must exist, all directory and its subdirs will be created as necessary

  [FEED]

    Read the feed from the specified file instead of stdin

    If "-" use stdin, otherwise must exist and be a file

## EXIT VALUES

    1 - option parsing error
    2 - unused
    3 - input file opening failure
    4 - output write failure
    5 - input read failure
    6 - unused
    7 - insufficient instruction data
    8 - unused
    9 - unused

## AUTHOR

Written by наб &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/feembox/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/feembox>&gt;

&lt;<https://cr.yp.to/proto/maildir.html>&gt;
