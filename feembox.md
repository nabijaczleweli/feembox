feembox(1) -- What if a feed, but it's a mailbox?
=================================================

## SYNOPSIS

`feembox` [-v] [-t FROM:TO:HOW]... [MAILDIR] [FEED]<br />
`feembox` [-v] [-t FROM:TO:HOW]... [MAILDIR] < feed.xml

## DESCRIPTION

`feembox` represents an (RSS/Atom/JSON) feed as a mailbox in the [maildir](https://cr.yp.to/proto/maildir.html) format.

## OPTIONS

  [MAILDIR]

    Deliver to the specified directory instead of the CWD

    Parents must exist, all directory and its subdirs will be created as necessary

  [FEED]

    Read the feed from the specified file instead of stdin

    If "-" use stdin, otherwise must exist and be a file

  -v --verbose

    Print what's happening to the standard output,
    if specified twice: print parse debugging information.

  -t --transfrom <FROM:TO:HOW|FROM;TO;HOW>...

    Define an alternative transformation invocation HOW
    from the mime-type FROM to the mime-type TO.

    If the post content type matches FROM, "/bin/sh -c HOW" ("cmd /C HOW" on NT)
    is executed, its standard input tied thereto, and standard output
    to the buffer for the new multipart/alternative part TO.

    The separator between FROM, TO, and HOW is the platform's path list separator
    (i.e. ";" on NT and ":" elsewhere).

    Can be specified multiple times, in which case each transformation is invoked once,
    in order, on the current set of parts.

## EXIT VALUES

    1 - option parse error
    2 - feed file open failed
    3 - feed parse failed
    4 - maildir subdirectory read failed
    5 - existing mail open(2)/mmap(2) failed
    6 - creating a MAILDIR/tmp or MAILDIR/new failed
    7 - formatting mail failed
    8 - creating/writing/delivering mail failed

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
