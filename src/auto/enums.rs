// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use error::ErrorDomain;
use glib_sys;
use std::fmt;
use translate::*;
use Quark;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ChecksumType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    #[cfg(any(feature = "v2_52", feature = "dox"))]
    Sha384,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ChecksumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ChecksumType::{}",
            match *self {
                ChecksumType::Md5 => "Md5",
                ChecksumType::Sha1 => "Sha1",
                ChecksumType::Sha256 => "Sha256",
                ChecksumType::Sha512 => "Sha512",
                #[cfg(any(feature = "v2_52", feature = "dox"))]
                ChecksumType::Sha384 => "Sha384",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ChecksumType {
    type GlibType = glib_sys::GChecksumType;

    fn to_glib(&self) -> glib_sys::GChecksumType {
        match *self {
            ChecksumType::Md5 => glib_sys::G_CHECKSUM_MD5,
            ChecksumType::Sha1 => glib_sys::G_CHECKSUM_SHA1,
            ChecksumType::Sha256 => glib_sys::G_CHECKSUM_SHA256,
            ChecksumType::Sha512 => glib_sys::G_CHECKSUM_SHA512,
            #[cfg(any(feature = "v2_52", feature = "dox"))]
            ChecksumType::Sha384 => glib_sys::G_CHECKSUM_SHA384,
            ChecksumType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GChecksumType> for ChecksumType {
    fn from_glib(value: glib_sys::GChecksumType) -> Self {
        match value {
            0 => ChecksumType::Md5,
            1 => ChecksumType::Sha1,
            2 => ChecksumType::Sha256,
            3 => ChecksumType::Sha512,
            #[cfg(any(feature = "v2_52", feature = "dox"))]
            4 => ChecksumType::Sha384,
            value => ChecksumType::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum DateMonth {
    BadMonth,
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DateMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DateMonth::{}",
            match *self {
                DateMonth::BadMonth => "BadMonth",
                DateMonth::January => "January",
                DateMonth::February => "February",
                DateMonth::March => "March",
                DateMonth::April => "April",
                DateMonth::May => "May",
                DateMonth::June => "June",
                DateMonth::July => "July",
                DateMonth::August => "August",
                DateMonth::September => "September",
                DateMonth::October => "October",
                DateMonth::November => "November",
                DateMonth::December => "December",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for DateMonth {
    type GlibType = glib_sys::GDateMonth;

    fn to_glib(&self) -> glib_sys::GDateMonth {
        match *self {
            DateMonth::BadMonth => glib_sys::G_DATE_BAD_MONTH,
            DateMonth::January => glib_sys::G_DATE_JANUARY,
            DateMonth::February => glib_sys::G_DATE_FEBRUARY,
            DateMonth::March => glib_sys::G_DATE_MARCH,
            DateMonth::April => glib_sys::G_DATE_APRIL,
            DateMonth::May => glib_sys::G_DATE_MAY,
            DateMonth::June => glib_sys::G_DATE_JUNE,
            DateMonth::July => glib_sys::G_DATE_JULY,
            DateMonth::August => glib_sys::G_DATE_AUGUST,
            DateMonth::September => glib_sys::G_DATE_SEPTEMBER,
            DateMonth::October => glib_sys::G_DATE_OCTOBER,
            DateMonth::November => glib_sys::G_DATE_NOVEMBER,
            DateMonth::December => glib_sys::G_DATE_DECEMBER,
            DateMonth::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GDateMonth> for DateMonth {
    fn from_glib(value: glib_sys::GDateMonth) -> Self {
        match value {
            0 => DateMonth::BadMonth,
            1 => DateMonth::January,
            2 => DateMonth::February,
            3 => DateMonth::March,
            4 => DateMonth::April,
            5 => DateMonth::May,
            6 => DateMonth::June,
            7 => DateMonth::July,
            8 => DateMonth::August,
            9 => DateMonth::September,
            10 => DateMonth::October,
            11 => DateMonth::November,
            12 => DateMonth::December,
            value => DateMonth::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum DateWeekday {
    BadWeekday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DateWeekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DateWeekday::{}",
            match *self {
                DateWeekday::BadWeekday => "BadWeekday",
                DateWeekday::Monday => "Monday",
                DateWeekday::Tuesday => "Tuesday",
                DateWeekday::Wednesday => "Wednesday",
                DateWeekday::Thursday => "Thursday",
                DateWeekday::Friday => "Friday",
                DateWeekday::Saturday => "Saturday",
                DateWeekday::Sunday => "Sunday",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for DateWeekday {
    type GlibType = glib_sys::GDateWeekday;

    fn to_glib(&self) -> glib_sys::GDateWeekday {
        match *self {
            DateWeekday::BadWeekday => glib_sys::G_DATE_BAD_WEEKDAY,
            DateWeekday::Monday => glib_sys::G_DATE_MONDAY,
            DateWeekday::Tuesday => glib_sys::G_DATE_TUESDAY,
            DateWeekday::Wednesday => glib_sys::G_DATE_WEDNESDAY,
            DateWeekday::Thursday => glib_sys::G_DATE_THURSDAY,
            DateWeekday::Friday => glib_sys::G_DATE_FRIDAY,
            DateWeekday::Saturday => glib_sys::G_DATE_SATURDAY,
            DateWeekday::Sunday => glib_sys::G_DATE_SUNDAY,
            DateWeekday::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GDateWeekday> for DateWeekday {
    fn from_glib(value: glib_sys::GDateWeekday) -> Self {
        match value {
            0 => DateWeekday::BadWeekday,
            1 => DateWeekday::Monday,
            2 => DateWeekday::Tuesday,
            3 => DateWeekday::Wednesday,
            4 => DateWeekday::Thursday,
            5 => DateWeekday::Friday,
            6 => DateWeekday::Saturday,
            7 => DateWeekday::Sunday,
            value => DateWeekday::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum KeyFileError {
    UnknownEncoding,
    Parse,
    NotFound,
    KeyNotFound,
    GroupNotFound,
    InvalidValue,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for KeyFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "KeyFileError::{}",
            match *self {
                KeyFileError::UnknownEncoding => "UnknownEncoding",
                KeyFileError::Parse => "Parse",
                KeyFileError::NotFound => "NotFound",
                KeyFileError::KeyNotFound => "KeyNotFound",
                KeyFileError::GroupNotFound => "GroupNotFound",
                KeyFileError::InvalidValue => "InvalidValue",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for KeyFileError {
    type GlibType = glib_sys::GKeyFileError;

    fn to_glib(&self) -> glib_sys::GKeyFileError {
        match *self {
            KeyFileError::UnknownEncoding => glib_sys::G_KEY_FILE_ERROR_UNKNOWN_ENCODING,
            KeyFileError::Parse => glib_sys::G_KEY_FILE_ERROR_PARSE,
            KeyFileError::NotFound => glib_sys::G_KEY_FILE_ERROR_NOT_FOUND,
            KeyFileError::KeyNotFound => glib_sys::G_KEY_FILE_ERROR_KEY_NOT_FOUND,
            KeyFileError::GroupNotFound => glib_sys::G_KEY_FILE_ERROR_GROUP_NOT_FOUND,
            KeyFileError::InvalidValue => glib_sys::G_KEY_FILE_ERROR_INVALID_VALUE,
            KeyFileError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GKeyFileError> for KeyFileError {
    fn from_glib(value: glib_sys::GKeyFileError) -> Self {
        match value {
            0 => KeyFileError::UnknownEncoding,
            1 => KeyFileError::Parse,
            2 => KeyFileError::NotFound,
            3 => KeyFileError::KeyNotFound,
            4 => KeyFileError::GroupNotFound,
            5 => KeyFileError::InvalidValue,
            value => KeyFileError::__Unknown(value),
        }
    }
}

impl ErrorDomain for KeyFileError {
    fn domain() -> Quark {
        unsafe { from_glib(glib_sys::g_key_file_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(KeyFileError::UnknownEncoding),
            1 => Some(KeyFileError::Parse),
            2 => Some(KeyFileError::NotFound),
            3 => Some(KeyFileError::KeyNotFound),
            4 => Some(KeyFileError::GroupNotFound),
            5 => Some(KeyFileError::InvalidValue),
            value => Some(KeyFileError::__Unknown(value)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum OptionArg {
    None,
    String,
    Int,
    Callback,
    Filename,
    StringArray,
    FilenameArray,
    Double,
    Int64,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for OptionArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OptionArg::{}",
            match *self {
                OptionArg::None => "None",
                OptionArg::String => "String",
                OptionArg::Int => "Int",
                OptionArg::Callback => "Callback",
                OptionArg::Filename => "Filename",
                OptionArg::StringArray => "StringArray",
                OptionArg::FilenameArray => "FilenameArray",
                OptionArg::Double => "Double",
                OptionArg::Int64 => "Int64",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for OptionArg {
    type GlibType = glib_sys::GOptionArg;

    fn to_glib(&self) -> glib_sys::GOptionArg {
        match *self {
            OptionArg::None => glib_sys::G_OPTION_ARG_NONE,
            OptionArg::String => glib_sys::G_OPTION_ARG_STRING,
            OptionArg::Int => glib_sys::G_OPTION_ARG_INT,
            OptionArg::Callback => glib_sys::G_OPTION_ARG_CALLBACK,
            OptionArg::Filename => glib_sys::G_OPTION_ARG_FILENAME,
            OptionArg::StringArray => glib_sys::G_OPTION_ARG_STRING_ARRAY,
            OptionArg::FilenameArray => glib_sys::G_OPTION_ARG_FILENAME_ARRAY,
            OptionArg::Double => glib_sys::G_OPTION_ARG_DOUBLE,
            OptionArg::Int64 => glib_sys::G_OPTION_ARG_INT64,
            OptionArg::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GOptionArg> for OptionArg {
    fn from_glib(value: glib_sys::GOptionArg) -> Self {
        match value {
            0 => OptionArg::None,
            1 => OptionArg::String,
            2 => OptionArg::Int,
            3 => OptionArg::Callback,
            4 => OptionArg::Filename,
            5 => OptionArg::StringArray,
            6 => OptionArg::FilenameArray,
            7 => OptionArg::Double,
            8 => OptionArg::Int64,
            value => OptionArg::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SeekType {
    Cur,
    Set,
    End,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SeekType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SeekType::{}",
            match *self {
                SeekType::Cur => "Cur",
                SeekType::Set => "Set",
                SeekType::End => "End",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SeekType {
    type GlibType = glib_sys::GSeekType;

    fn to_glib(&self) -> glib_sys::GSeekType {
        match *self {
            SeekType::Cur => glib_sys::G_SEEK_CUR,
            SeekType::Set => glib_sys::G_SEEK_SET,
            SeekType::End => glib_sys::G_SEEK_END,
            SeekType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GSeekType> for SeekType {
    fn from_glib(value: glib_sys::GSeekType) -> Self {
        match value {
            0 => SeekType::Cur,
            1 => SeekType::Set,
            2 => SeekType::End,
            value => SeekType::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TimeType {
    Standard,
    Daylight,
    Universal,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TimeType::{}",
            match *self {
                TimeType::Standard => "Standard",
                TimeType::Daylight => "Daylight",
                TimeType::Universal => "Universal",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TimeType {
    type GlibType = glib_sys::GTimeType;

    fn to_glib(&self) -> glib_sys::GTimeType {
        match *self {
            TimeType::Standard => glib_sys::G_TIME_TYPE_STANDARD,
            TimeType::Daylight => glib_sys::G_TIME_TYPE_DAYLIGHT,
            TimeType::Universal => glib_sys::G_TIME_TYPE_UNIVERSAL,
            TimeType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<glib_sys::GTimeType> for TimeType {
    fn from_glib(value: glib_sys::GTimeType) -> Self {
        match value {
            0 => TimeType::Standard,
            1 => TimeType::Daylight,
            2 => TimeType::Universal,
            value => TimeType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum UriError {
    Failed,
    BadScheme,
    BadUser,
    BadPassword,
    BadAuthParams,
    BadHost,
    BadPort,
    BadPath,
    BadQuery,
    BadFragment,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UriError::{}",
            match *self {
                UriError::Failed => "Failed",
                UriError::BadScheme => "BadScheme",
                UriError::BadUser => "BadUser",
                UriError::BadPassword => "BadPassword",
                UriError::BadAuthParams => "BadAuthParams",
                UriError::BadHost => "BadHost",
                UriError::BadPort => "BadPort",
                UriError::BadPath => "BadPath",
                UriError::BadQuery => "BadQuery",
                UriError::BadFragment => "BadFragment",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for UriError {
    type GlibType = glib_sys::GUriError;

    fn to_glib(&self) -> glib_sys::GUriError {
        match *self {
            UriError::Failed => glib_sys::G_URI_ERROR_FAILED,
            UriError::BadScheme => glib_sys::G_URI_ERROR_BAD_SCHEME,
            UriError::BadUser => glib_sys::G_URI_ERROR_BAD_USER,
            UriError::BadPassword => glib_sys::G_URI_ERROR_BAD_PASSWORD,
            UriError::BadAuthParams => glib_sys::G_URI_ERROR_BAD_AUTH_PARAMS,
            UriError::BadHost => glib_sys::G_URI_ERROR_BAD_HOST,
            UriError::BadPort => glib_sys::G_URI_ERROR_BAD_PORT,
            UriError::BadPath => glib_sys::G_URI_ERROR_BAD_PATH,
            UriError::BadQuery => glib_sys::G_URI_ERROR_BAD_QUERY,
            UriError::BadFragment => glib_sys::G_URI_ERROR_BAD_FRAGMENT,
            UriError::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<glib_sys::GUriError> for UriError {
    fn from_glib(value: glib_sys::GUriError) -> Self {
        match value {
            0 => UriError::Failed,
            1 => UriError::BadScheme,
            2 => UriError::BadUser,
            3 => UriError::BadPassword,
            4 => UriError::BadAuthParams,
            5 => UriError::BadHost,
            6 => UriError::BadPort,
            7 => UriError::BadPath,
            8 => UriError::BadQuery,
            9 => UriError::BadFragment,
            value => UriError::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
impl ErrorDomain for UriError {
    fn domain() -> Quark {
        unsafe { from_glib(glib_sys::g_uri_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(UriError::Failed),
            1 => Some(UriError::BadScheme),
            2 => Some(UriError::BadUser),
            3 => Some(UriError::BadPassword),
            4 => Some(UriError::BadAuthParams),
            5 => Some(UriError::BadHost),
            6 => Some(UriError::BadPort),
            7 => Some(UriError::BadPath),
            8 => Some(UriError::BadQuery),
            9 => Some(UriError::BadFragment),
            _ => Some(UriError::Failed),
        }
    }
}
