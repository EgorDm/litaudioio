use std::error;
use std::ffi::CStr;
use std::fmt;
use std::io;
use std::str::from_utf8_unchecked;

use super::unix_errors::*;
use crate::sys::*;
use libc::c_int;

#[derive(Copy, Clone)]
pub enum FFError {
    Again,
    Bug,
    Bug2,
    Unknown,
    Experimental,
    BufferTooSmall,
    Eof,
    Exit,
    External,
    InvalidData,
    PatchWelcome,

    InputChanged,
    OutputChanged,

    BsfNotFound,
    DecoderNotFound,
    DemuxerNotFound,
    EncoderNotFound,
    OptionNotFound,
    MuxerNotFound,
    FilterNotFound,
    ProtocolNotFound,
    StreamNotFound,

    HttpBadRequest,
    HttpUnauthorized,
    HttpForbidden,
    HttpNotFound,
    HttpOther4xx,
    HttpServerError,
    Unix(c_int),
}

#[allow(non_snake_case)]
#[inline(always)]
pub const fn AVERRORL(e: c_int) -> c_int {
    -e
}

pub const AVERROR_EAGAIN: c_int = AVERRORL(EAGAIN);
pub const AVERROR_ENOMEM: c_int = AVERRORL(ENOMEM);
pub const AVERROR_EINVAL: c_int = AVERRORL(EINVAL);

impl From<c_int> for FFError {
    fn from(value: c_int) -> FFError {
        match value {
            AVERROR_BSF_NOT_FOUND => FFError::BsfNotFound,
            AVERROR_BUG => FFError::Bug,
            AVERROR_BUFFER_TOO_SMALL => FFError::BufferTooSmall,
            AVERROR_DECODER_NOT_FOUND => FFError::DecoderNotFound,
            AVERROR_DEMUXER_NOT_FOUND => FFError::DemuxerNotFound,
            AVERROR_ENCODER_NOT_FOUND => FFError::EncoderNotFound,
            AVERROR_EOF => FFError::Eof,
            AVERROR_EXIT => FFError::Exit,
            AVERROR_EXTERNAL => FFError::External,
            AVERROR_FILTER_NOT_FOUND => FFError::FilterNotFound,
            AVERROR_INVALIDDATA => FFError::InvalidData,
            AVERROR_MUXER_NOT_FOUND => FFError::MuxerNotFound,
            AVERROR_OPTION_NOT_FOUND => FFError::OptionNotFound,
            AVERROR_PATCHWELCOME => FFError::PatchWelcome,
            AVERROR_PROTOCOL_NOT_FOUND => FFError::ProtocolNotFound,
            AVERROR_STREAM_NOT_FOUND => FFError::StreamNotFound,
            AVERROR_BUG2 => FFError::Bug2,
            AVERROR_UNKNOWN => FFError::Unknown,
            AVERROR_EXPERIMENTAL => FFError::Experimental,
            AVERROR_INPUT_CHANGED => FFError::InputChanged,
            AVERROR_OUTPUT_CHANGED => FFError::OutputChanged,
            AVERROR_HTTP_BAD_REQUEST => FFError::HttpBadRequest,
            AVERROR_HTTP_UNAUTHORIZED => FFError::HttpUnauthorized,
            AVERROR_HTTP_FORBIDDEN => FFError::HttpForbidden,
            AVERROR_HTTP_NOT_FOUND => FFError::HttpNotFound,
            AVERROR_HTTP_OTHER_4XX => FFError::HttpOther4xx,
            AVERROR_HTTP_SERVER_ERROR => FFError::HttpServerError,
            AVERROR_EAGAIN => FFError::Again,

            v => {
                if AVERRORL(v) < 128 {
                    FFError::Unix(AVERRORL(v))
                } else {
                    FFError::Unknown
                }
            },
        }
    }
}

impl Into<c_int> for FFError {
    fn into(self) -> c_int {
        match self {
            FFError::BsfNotFound => AVERROR_BSF_NOT_FOUND,
            FFError::Bug => AVERROR_BUG,
            FFError::BufferTooSmall => AVERROR_BUFFER_TOO_SMALL,
            FFError::DecoderNotFound => AVERROR_DECODER_NOT_FOUND,
            FFError::DemuxerNotFound => AVERROR_DEMUXER_NOT_FOUND,
            FFError::EncoderNotFound => AVERROR_ENCODER_NOT_FOUND,
            FFError::Eof => AVERROR_EOF,
            FFError::Exit => AVERROR_EXIT,
            FFError::External => AVERROR_EXTERNAL,
            FFError::FilterNotFound => AVERROR_FILTER_NOT_FOUND,
            FFError::InvalidData => AVERROR_INVALIDDATA,
            FFError::MuxerNotFound => AVERROR_MUXER_NOT_FOUND,
            FFError::OptionNotFound => AVERROR_OPTION_NOT_FOUND,
            FFError::PatchWelcome => AVERROR_PATCHWELCOME,
            FFError::ProtocolNotFound => AVERROR_PROTOCOL_NOT_FOUND,
            FFError::StreamNotFound => AVERROR_STREAM_NOT_FOUND,
            FFError::Bug2 => AVERROR_BUG2,
            FFError::Unknown => AVERROR_UNKNOWN,
            FFError::Experimental => AVERROR_EXPERIMENTAL,
            FFError::InputChanged => AVERROR_INPUT_CHANGED,
            FFError::OutputChanged => AVERROR_OUTPUT_CHANGED,
            FFError::HttpBadRequest => AVERROR_HTTP_BAD_REQUEST,
            FFError::HttpUnauthorized => AVERROR_HTTP_UNAUTHORIZED,
            FFError::HttpForbidden => AVERROR_HTTP_FORBIDDEN,
            FFError::HttpNotFound => AVERROR_HTTP_NOT_FOUND,
            FFError::HttpOther4xx => AVERROR_HTTP_OTHER_4XX,
            FFError::HttpServerError => AVERROR_HTTP_SERVER_ERROR,
            FFError::Again => AVERROR_EAGAIN,
            FFError::Unix(v) => AVERRORL(v)
        }
    }
}

impl From<FFError> for io::Error {
    fn from(value: FFError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, value)
    }
}

impl fmt::Display for FFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(error::Error::description(self))
    }
}

impl fmt::Debug for FFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("ffmpeg::Error(")?;
        f.write_str(&format!("{}: ", AVUNERROR((*self).into())))?;
        fmt::Display::fmt(self, f)?;
        f.write_str(")")
    }
}

#[inline(always)]
fn index(error: &FFError) -> usize {
    match *error {
        FFError::BsfNotFound => 0,
        FFError::Bug => 1,
        FFError::BufferTooSmall => 2,
        FFError::DecoderNotFound => 3,
        FFError::DemuxerNotFound => 4,
        FFError::EncoderNotFound => 5,
        FFError::Eof => 6,
        FFError::Exit => 7,
        FFError::External => 8,
        FFError::FilterNotFound => 9,
        FFError::InvalidData => 10,
        FFError::MuxerNotFound => 11,
        FFError::OptionNotFound => 12,
        FFError::PatchWelcome => 13,
        FFError::ProtocolNotFound => 14,
        FFError::StreamNotFound => 15,
        FFError::Bug2 => 16,
        FFError::Unknown => 17,
        FFError::Experimental => 18,
        FFError::InputChanged => 19,
        FFError::OutputChanged => 20,
        FFError::HttpBadRequest => 21,
        FFError::HttpUnauthorized => 22,
        FFError::HttpForbidden => 23,
        FFError::HttpNotFound => 24,
        FFError::HttpOther4xx => 25,
        FFError::HttpServerError => 26,
        FFError::Again => 27,
        FFError::Unix(_) => 0,
    }
}

// XXX: the length has to be synced with the number of errors
static mut STRINGS: [[i8; AV_ERROR_MAX_STRING_SIZE as usize]; 28] =
    [[0i8; AV_ERROR_MAX_STRING_SIZE as usize]; 28];

pub fn register_all() {
    unsafe {
        av_strerror(
            FFError::Bug.into(),
            STRINGS[index(&FFError::Bug)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::Bug2.into(),
            STRINGS[index(&FFError::Bug2)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::Unknown.into(),
            STRINGS[index(&FFError::Unknown)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::Experimental.into(),
            STRINGS[index(&FFError::Experimental)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::BufferTooSmall.into(),
            STRINGS[index(&FFError::BufferTooSmall)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::Eof.into(),
            STRINGS[index(&FFError::Eof)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::Exit.into(),
            STRINGS[index(&FFError::Exit)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::External.into(),
            STRINGS[index(&FFError::External)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::InvalidData.into(),
            STRINGS[index(&FFError::InvalidData)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::PatchWelcome.into(),
            STRINGS[index(&FFError::PatchWelcome)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );

        av_strerror(
            FFError::InputChanged.into(),
            STRINGS[index(&FFError::InputChanged)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::OutputChanged.into(),
            STRINGS[index(&FFError::OutputChanged)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );

        av_strerror(
            FFError::BsfNotFound.into(),
            STRINGS[index(&FFError::BsfNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::DecoderNotFound.into(),
            STRINGS[index(&FFError::DecoderNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::DemuxerNotFound.into(),
            STRINGS[index(&FFError::DemuxerNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::EncoderNotFound.into(),
            STRINGS[index(&FFError::EncoderNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::OptionNotFound.into(),
            STRINGS[index(&FFError::OptionNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::MuxerNotFound.into(),
            STRINGS[index(&FFError::MuxerNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::FilterNotFound.into(),
            STRINGS[index(&FFError::FilterNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::ProtocolNotFound.into(),
            STRINGS[index(&FFError::ProtocolNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::StreamNotFound.into(),
            STRINGS[index(&FFError::StreamNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );

        av_strerror(
            FFError::HttpBadRequest.into(),
            STRINGS[index(&FFError::HttpBadRequest)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::HttpUnauthorized.into(),
            STRINGS[index(&FFError::HttpUnauthorized)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::HttpForbidden.into(),
            STRINGS[index(&FFError::HttpForbidden)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::HttpNotFound.into(),
            STRINGS[index(&FFError::HttpNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::HttpOther4xx.into(),
            STRINGS[index(&FFError::HttpOther4xx)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            FFError::HttpServerError.into(),
            STRINGS[index(&FFError::HttpServerError)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
    }
}

impl error::Error for FFError {
    fn description(&self) -> &str {
        match self {
            FFError::Unix(v) => {
                unix_err_to_string(*v)
            },
            v => {
                unsafe { from_utf8_unchecked(CStr::from_ptr(STRINGS[index(v)].as_ptr()).to_bytes()) }
            }

        }

    }
}


