// Copyright 2016  Jonas Me
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate fast_chemail;

mod table;

#[test]
fn parse_email() {
    let mut idx = 0;
    for x in &table::TESTS_OK {
        idx += 1;
        // println!("{}", idx);
        assert_eq!(fast_chemail::parse_email(x), Ok(())) ;
    }

    idx = 0;
    for x in &table::TESTS_ERROR[..] {
        idx += 1;
        // println!("{}", idx);
        match fast_chemail::parse_email(x.0) {
            Err(e) => assert_eq!(e, x.1),
            Ok(_) => panic!("[{}] `{}` want error `{:?}`", idx, x.0, x.1),
        }
    }
}

#[test]
fn is_valid_email() {
    assert_eq!(fast_chemail::is_valid_email(&table::TESTS_OK[0]), true);
    assert_eq!(fast_chemail::is_valid_email(&table::TESTS_ERROR[0].0), false);
}
