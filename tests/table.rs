// Copyright 2016  Jonas Me
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate asciiutils;
extern crate fastchemail;

use fastchemail::ParseError;

pub static TESTS_OK: [&'static str; 16] = ["!#$%&'*+-/=?^_`{|}~@example.com",
                                           "user+mailbox@example.com",
                                           "customer/department=shipping@example.com",
                                           "$A12345@example.com",
                                           "!def!xyz%abc@example.com",
                                           "_somename@example.com",
                                           "a@example.com",
                                           "a@x.y",
                                           "abc.def@example.com",
                                           "abc-def@example.com",
                                           "123@example.com",
                                           "xn--abc@example.com",
                                           "abc@x.y.z",
                                           "abc@xyz-example.com",
                                           "abc@c--n.com",
                                           "abc@xn--hxajbheg2az3al.xn--jxalpdlp"];

pub static TESTS_ERROR: [(&'static str, ParseError); 29] =
    [("@", ParseError::NoLocalPart),
     ("@example.com", ParseError::NoLocalPart),
     ("abc@", ParseError::NoDomainPart),
     ("abc", ParseError::NoSignAt),
     ("abc@def@example.com", ParseError::TooAt),
     (".abc@example.com", ParseError::LocalStartPeriod),
     ("abc.@example.com", ParseError::LocalEndPeriod),
     ("abc@.example.com", ParseError::DomainStartPeriod),
     ("abc@example.com.", ParseError::DomainEndPeriod),
     ("ab..cd@example.com", ParseError::ConsecutivePeriod),
     ("abc@example..com", ParseError::ConsecutivePeriod),
     ("a@example", ParseError::NoPeriodDomain),
     (r#"ab\c@example.com"#, ParseError::WrongCharLocal('\\')),
     (r#"ab"c"def@example.com"#, ParseError::WrongCharLocal('"')),
     ("abc def@example.com", ParseError::WrongCharLocal(' ')),
     ("(comment)abc@example.com", ParseError::WrongCharLocal('(')),
     ("abc@[255.255.255.255]", ParseError::WrongCharDomain('[')),
     ("abc@(example.com", ParseError::WrongCharDomain('(')),
     ("abc@x.y_y.z", ParseError::WrongCharDomain('_')),
     ("abc@-example.com", ParseError::WrongStartLabel('-')),
     ("abc@example-.com", ParseError::WrongEndLabel('-')),
     ("abc@x.-y.z", ParseError::WrongStartLabel('-')),
     ("abc@x.y-.z", ParseError::WrongEndLabel('-')),
     ("abc@1example.com", ParseError::WrongStartLabel('1')),
     ("abc@x.123", ParseError::WrongStartLabel('1')),
     ("abcd€f@example.com", ParseError::Ascii(asciiutils::AsciiError::NonAscii('€'))),
     ("abc@exámple.com", ParseError::Ascii(asciiutils::AsciiError::NonAscii('á'))),
     ("a\tbc@example.com", ParseError::Ascii(asciiutils::AsciiError::ControlChar(2))),
     ("abc@\texample.com", ParseError::Ascii(asciiutils::AsciiError::ControlChar(5)))];
