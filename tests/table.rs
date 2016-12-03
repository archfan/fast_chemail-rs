// Copyright 2016  Jonas Me
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate asciiutils;
extern crate fastchemail;

use fastchemail::EmailError;

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

pub static TESTS_ERROR: [(&'static str, EmailError); 29] =
    [("@", EmailError::NoLocalPart),
     ("@example.com", EmailError::NoLocalPart),
     ("abc@", EmailError::NoDomainPart),
     ("abc", EmailError::NoSignAt),
     ("abc@def@example.com", EmailError::TooAt),
     (".abc@example.com", EmailError::LocalStartPeriod),
     ("abc.@example.com", EmailError::LocalEndPeriod),
     ("abc@.example.com", EmailError::DomainStartPeriod),
     ("abc@example.com.", EmailError::DomainEndPeriod),
     ("ab..cd@example.com", EmailError::ConsecutivePeriod),
     ("abc@example..com", EmailError::ConsecutivePeriod),
     ("a@example", EmailError::NoPeriodDomain),
     (r#"ab\c@example.com"#, EmailError::WrongCharLocal('\\')),
     (r#"ab"c"def@example.com"#, EmailError::WrongCharLocal('"')),
     ("abc def@example.com", EmailError::WrongCharLocal(' ')),
     ("(comment)abc@example.com", EmailError::WrongCharLocal('(')),
     ("abc@[255.255.255.255]", EmailError::WrongCharDomain('[')),
     ("abc@(example.com", EmailError::WrongCharDomain('(')),
     ("abc@x.y_y.z", EmailError::WrongCharDomain('_')),
     ("abc@-example.com", EmailError::WrongStartLabel('-')),
     ("abc@example-.com", EmailError::WrongEndLabel('-')),
     ("abc@x.-y.z", EmailError::WrongStartLabel('-')),
     ("abc@x.y-.z", EmailError::WrongEndLabel('-')),
     ("abc@1example.com", EmailError::WrongStartLabel('1')),
     ("abc@x.123", EmailError::WrongStartLabel('1')),
     ("abcd€f@example.com", EmailError::Ascii(asciiutils::AsciiError::NonAscii('€'))),
     ("abc@exámple.com", EmailError::Ascii(asciiutils::AsciiError::NonAscii('á'))),
     ("a\tbc@example.com", EmailError::Ascii(asciiutils::AsciiError::ControlChar(2))),
     ("abc@\texample.com", EmailError::Ascii(asciiutils::AsciiError::ControlChar(5)))];
