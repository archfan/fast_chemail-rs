// Copyright 2016  Jonas Me
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error;
use std::error::Error as ErrorT;
use std::fmt;

use ascii_utils;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NoLocalPart,
    NoDomainPart,
    NoSignAt,

    TooAt,
    LocalTooLong,
    DomainTooLong,
    LabelTooLong,

    LocalStartPeriod,
    LocalEndPeriod,
    DomainStartPeriod,
    DomainEndPeriod,
    ConsecutivePeriod,
    NoPeriodDomain,

    Ascii(ascii_utils::AsciiError),
    WrongCharLocal(char),
    WrongCharDomain(char),
    WrongStartLabel(char),
    WrongEndLabel(char),
}

impl From<ascii_utils::AsciiError> for ParseError {
    fn from(err: ascii_utils::AsciiError) -> ParseError {
        ParseError::Ascii(err)
    }
}

impl error::Error for ParseError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseError::Ascii(ref err) => Some(err),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        match *self {
            ParseError::NoLocalPart => "no local part",
            ParseError::NoDomainPart => "no domain part",
            ParseError::NoSignAt => "no at sign (@)",

            ParseError::TooAt => "wrong number of at sign (@)",
            ParseError::LocalTooLong => "the local part has more than 64 characters",
            ParseError::DomainTooLong => "the domain part has more than 255 characters",
            ParseError::LabelTooLong => "a domain label has more than 63 characters",

            ParseError::LocalStartPeriod => "the local part starts with a period",
            ParseError::LocalEndPeriod => "the local part ends with a period",
            ParseError::DomainStartPeriod => "the domain part starts with a period",
            ParseError::DomainEndPeriod => "the domain part ends with a period",
            ParseError::ConsecutivePeriod => "appear two or more consecutive periods",
            ParseError::NoPeriodDomain => "no period at domain part",

            ParseError::Ascii(ref err) => err.description(),
            ParseError::WrongCharLocal(_) => "character not valid in local part",
            ParseError::WrongCharDomain(_) => "character not valid in domain part",
            ParseError::WrongStartLabel(_) => "character not valid at start of domain label",
            ParseError::WrongEndLabel(_) => "character not valid at end of domain label",
        }
    }
}

const MSG_ERR: &'static str = "invalid email address";
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::WrongCharLocal(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }
            ParseError::WrongCharDomain(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }
            ParseError::WrongStartLabel(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }
            ParseError::WrongEndLabel(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }

            _ => write!(f, "{}: {}", MSG_ERR, self.description()),
        }
    }
}
