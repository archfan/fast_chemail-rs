// Copyright 2016  Jonas Me
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate asciiutils;
extern crate fastchemail;

use fastchemail::Error;

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

pub static TESTS_ERROR: [(&'static str, Error); 29] =
    [("@", Error::NoLocalPart),
     ("@example.com", Error::NoLocalPart),
     ("abc@", Error::NoDomainPart),
     ("abc", Error::NoSignAt),
     ("abc@def@example.com", Error::TooAt),
     (".abc@example.com", Error::LocalStartPeriod),
     ("abc.@example.com", Error::LocalEndPeriod),
     ("abc@.example.com", Error::DomainStartPeriod),
     ("abc@example.com.", Error::DomainEndPeriod),
     ("ab..cd@example.com", Error::ConsecutivePeriod),
     ("abc@example..com", Error::ConsecutivePeriod),
     ("a@example", Error::NoPeriodDomain),
     (r#"ab\c@example.com"#, Error::WrongCharLocal('\\')),
     (r#"ab"c"def@example.com"#, Error::WrongCharLocal('"')),
     ("abc def@example.com", Error::WrongCharLocal(' ')),
     ("(comment)abc@example.com", Error::WrongCharLocal('(')),
     ("abc@[255.255.255.255]", Error::WrongCharDomain('[')),
     ("abc@(example.com", Error::WrongCharDomain('(')),
     ("abc@x.y_y.z", Error::WrongCharDomain('_')),
     ("abc@-example.com", Error::WrongStartLabel('-')),
     ("abc@example-.com", Error::WrongEndLabel('-')),
     ("abc@x.-y.z", Error::WrongStartLabel('-')),
     ("abc@x.y-.z", Error::WrongEndLabel('-')),
     ("abc@1example.com", Error::WrongStartLabel('1')),
     ("abc@x.123", Error::WrongStartLabel('1')),
     ("abcd€f@example.com", Error::Ascii(asciiutils::AsciiError::NonAscii('€'))),
     ("abc@exámple.com", Error::Ascii(asciiutils::AsciiError::NonAscii('á'))),
     ("a\tbc@example.com", Error::Ascii(asciiutils::AsciiError::ControlChar(2))),
     ("abc@\texample.com", Error::Ascii(asciiutils::AsciiError::ControlChar(5)))];
