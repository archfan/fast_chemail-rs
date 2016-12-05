# Fast Chemail

> A Dart library to validate the email as it is defined in the
> [HTML specification](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address).
> The RFC 5322 is too lax (allowing comments, whitespace characters, and quoted
> strings in manners unfamiliar to most users) to be of practical use.

Also, has been considered the bounds defined in:

- [SMTP :: Size Limits and Minimums](https://tools.ietf.org/html/rfc5321#section-4.5.3.1)

    > To the maximum extent possible, implementation techniques that impose no
    > limits on the length of these objects should be used.
    >
    > 1. The maximum total length of a user name or other local-part is 64 octets.
    > 2. The maximum total length of a domain name or number is 255 octets.
    > 3. The maximum total length of a reverse-path or forward-path is 256 octets
    >    (including the punctuation and element separators).

    There is a great contradiction. The maximum total length of an email address
    is 320 octects:

    `{64}@{255} => 64 + 1 + 255 = 320`

    but the maximum total length of a reverse-path or forward-path is 256 octets.
    Addition, that is only the length of the email address because it may
    contain a display name, like `first last <local@domain>` and will often
    extend 320 octets.

    Now well, since the standard says: "to the maximum extent possible,
    implementation techniques that impose no limits on the length of these
    objects should be used", the bound to reverse-path or forward-path will not
    be taken.

- [Application Techniques for Checking and Transformation of Names](https://tools.ietf.org/html/rfc3696#section-3)

    Such memo is not a standard else informational, so the correction done in
    the [errata](https://www.rfc-editor.org/errata_search.php?rfc=3696&eid=1690)
    is not followed.

## Table of Contents

- [Installation](#install)
- [Usage](#usage)
- [Contribute](#contribute)
- [License](#license)

## Install

Add to your 'Cargo.toml' file:

```toml
[dependencies]
fastchemail = "^0"
```

and to your crate root:

```rust
extern crate fastchemail;
```

## Usage

Read the [documentation](https://docs.rs/fastchemail).

## Contribute

The canonical source of this repository is hosted on
[GitLab](https://gitlab.com/fastchemail/fastchemail-rs).
Please make your issues there.

The pull requests are not accepted without filling an issue, to know wheter
what you want to change, it seems to me right for this project.

**Note:** this is a free/open source project at *zero price* built in my spare
time, and it is possible that I have not time to solve any issue.

## License

© 2016  Jonas Me
See the 'AUTHORS.md' file for a full list of authors.

The source files are distributed under the terms of the license
[Mozilla Public License, version 2.0](https://www.mozilla.org/en-US/MPL/2.0/)
