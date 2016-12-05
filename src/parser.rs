// Copyright 2016  Jonas Me
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error;
use std::error::Error as ErrorT;
use std::fmt;

use asciiutils;

// https://tools.ietf.org/html/rfc5321#section-4.5.3.1
//
// The maximum total length of a user name or other local-part is 64 octets.
// The maximum total length of a domain name or number is 255 octets.
const MAX_LOCAL_PART: usize = 64;
const MAX_DOMAIN_PART: usize = 255;
const MAX_LABEL: usize = 63;

/// `is_valid_email` checks wheter an email address is valid.
pub fn is_valid_email(address: &str) -> bool {
    parse_email(address).is_ok()
}

/// `parse_email` scans an email address to check wheter it is correct.
pub fn parse_email(address: &str) -> Result<(), EmailError> {
    if address.starts_with('@') {
        return Err(EmailError::NoLocalPart);
    }
    if address.ends_with('@') {
        return Err(EmailError::NoDomainPart);
    }

    // https://tools.ietf.org/html/rfc5321#section-4.1.2
    //
    // Systems MUST NOT define mailboxes in such a way as to require the use in
    // SMTP of non-ASCII characters (octets with the high order bit set to one)
    // or ASCII "control characters" (decimal value 0-31 and 127).
    asciiutils::check_ascii_printable(address)?;

    let mut address_iter = address.split('@');
    let local = address_iter.next().unwrap();
    let domain = address_iter.next().ok_or(EmailError::NoSignAt)?;
    if address_iter.next().is_some() {
        return Err(EmailError::TooAt);
    }

    // == Local part

    // https://tools.ietf.org/html/rfc3696#section-3
    //
    // Period (".") may also appear, but may not be used to start or end
    // the local part, nor may two or more consecutive periods appear.

    if local.len() > MAX_LOCAL_PART {
        return Err(EmailError::LocalTooLong);
    }
    if local.starts_with('.') {
        return Err(EmailError::LocalStartPeriod);
    }
    if local.ends_with('.') {
        return Err(EmailError::LocalEndPeriod);
    }

    let mut last_period: bool = false;
    for ch in local.chars() {
        if asciiutils::Check::is_letter(ch) || asciiutils::Check::is_digit(ch) {
            if last_period {
                last_period = false;
            }
            continue;
        }

        match ch {
            // atom :: https://tools.ietf.org/html/rfc5322#section-3.2.3
            '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | '-' | '/' | '=' | '?' | '^' |
            '_' | '`' | '{' | '|' | '}' | '~' => {
                if last_period {
                    last_period = false;
                }
            }
            '.' => {
                if last_period {
                    return Err(EmailError::ConsecutivePeriod);
                }
                last_period = true;
            }
            _ => return Err(EmailError::WrongCharLocal(ch)),
        }
    }

    // == Domain part

    // https://tools.ietf.org/html/rfc5321#section-4.1.2
    //
    // characters outside the set of alphabetic characters, digits, and hyphen
    // MUST NOT appear in domain name labels for SMTP clients or servers.  In
    // particular, the underscore character is not permitted.

    // https://tools.ietf.org/html/rfc1034#section-3.5
    //
    // The labels must follow the rules for ARPANET host names.  They must start
    // with a letter, end with a letter or digit, and have as interior
    // characters only letters, digits, and hyphen.  There are also some
    // restrictions on the length.  Labels must be 63 characters or less.

    // https://tools.ietf.org/html/rfc3696#section-2
    //
    // It is likely that the better strategy has now become to make the "at
    // least one period" test, to verify LDH conformance (including verification
    // that the apparent TLD name is not all-numeric), and then to use the DNS
    // to determine domain name validity, rather than trying to maintain a local
    // list of valid TLD names.

    if domain.len() > MAX_DOMAIN_PART {
        return Err(EmailError::DomainTooLong);
    }
    if domain.starts_with('.') {
        return Err(EmailError::DomainStartPeriod);
    }
    if domain.ends_with('.') {
        return Err(EmailError::DomainEndPeriod);
    }

    let labels: Vec<&str> = domain.split('.').collect();
    if labels.len() == 1 {
        return Err(EmailError::NoPeriodDomain);
    }

    for label in labels {
        if label.is_empty() {
            return Err(EmailError::ConsecutivePeriod);
        }
        if label.len() > MAX_LABEL {
            return Err(EmailError::LabelTooLong);
        }

        if let Some(ch) = label.chars().find(|&x| {
            !asciiutils::Check::is_letter(x) && !asciiutils::Check::is_digit(x) && x != '-'
        }) {
            return Err(EmailError::WrongCharDomain(ch));
        }

        let label_bytes = label.as_bytes();

        if !asciiutils::Check::is_letter(label_bytes[0]) {
            return Err(EmailError::WrongStartLabel(label_bytes[0] as char));
        }
        let last_char = label_bytes[label_bytes.len() - 1];
        if !asciiutils::Check::is_letter(last_char) && !asciiutils::Check::is_digit(last_char) {
            return Err(EmailError::WrongEndLabel(last_char as char));
        }
    }

    Ok(())
}

// == Errors
//

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
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

    Ascii(asciiutils::AsciiError),
    WrongCharLocal(char),
    WrongCharDomain(char),
    WrongStartLabel(char),
    WrongEndLabel(char),
}

impl error::Error for EmailError {
    fn description(&self) -> &str {
        match *self {
            EmailError::NoLocalPart => "no local part",
            EmailError::NoDomainPart => "no domain part",
            EmailError::NoSignAt => "no at sign (@)",

            EmailError::TooAt => "wrong number of at sign (@)",
            EmailError::LocalTooLong => "the local part has more than 64 characters",
            EmailError::DomainTooLong => "the domain part has more than 255 characters",
            EmailError::LabelTooLong => "a domain label has more than 63 characters",

            EmailError::LocalStartPeriod => "the local part starts with a period",
            EmailError::LocalEndPeriod => "the local part ends with a period",
            EmailError::DomainStartPeriod => "the domain part starts with a period",
            EmailError::DomainEndPeriod => "the domain part ends with a period",
            EmailError::ConsecutivePeriod => "appear two or more consecutive periods",
            EmailError::NoPeriodDomain => "no period at domain part",

            EmailError::Ascii(ref err) => err.description(),
            EmailError::WrongCharLocal(_) => "character not valid in local part",
            EmailError::WrongCharDomain(_) => "character not valid in domain part",
            EmailError::WrongStartLabel(_) => "character not valid at start of domain label",
            EmailError::WrongEndLabel(_) => "character not valid at end of domain label",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EmailError::Ascii(ref err) => Some(err),
            _ => None,
        }
    }
}

const MSG_ERR: &'static str = "invalid email address";
impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmailError::WrongCharLocal(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }
            EmailError::WrongCharDomain(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }
            EmailError::WrongStartLabel(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }
            EmailError::WrongEndLabel(ch) => {
                write!(f, "{}: {} ({})", MSG_ERR, self.description(), ch)
            }

            _ => write!(f, "{}: {}", MSG_ERR, self.description()),
        }
    }
}

impl From<asciiutils::AsciiError> for EmailError {
    fn from(err: asciiutils::AsciiError) -> EmailError {
        EmailError::Ascii(err)
    }
}

// == Tests
//

#[test]
fn test_length() {
    let local_part = "a".repeat(MAX_LOCAL_PART);

    let label = format!("{}.", "x".repeat(MAX_LABEL));
    let all_labels = format!("{}", label.repeat(3));
    let last_label = "y".repeat(MAX_DOMAIN_PART - all_labels.len());

    let input_ok = format!("{}@{}{}", local_part, all_labels, last_label);
    assert_eq!(parse_email(&input_ok), Ok(()));

    // == Errors

    let mut input_err = format!("a{}@{}{}", local_part, all_labels, last_label);
    assert_eq!(parse_email(&input_err), Err(EmailError::LocalTooLong));

    input_err = format!("{}@{}{}z", local_part, all_labels, last_label);
    assert_eq!(parse_email(&input_err), Err(EmailError::DomainTooLong));

    input_err = format!("{}@{}x{}", local_part, label, last_label);
    assert_eq!(parse_email(&input_err), Err(EmailError::LabelTooLong));
}
