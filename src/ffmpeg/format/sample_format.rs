use std::ffi::{CStr, CString};
use crate::sys::AVSampleFormat::*;
use crate::sys::*;
use std::str::from_utf8_unchecked;
use litaudio::format::*;
use litaudio::{AudioContainer, AudioStorageMut, AudioStorage};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SampleFormat {
    None,

    U8(Type),
    I16(Type),
    I32(Type),
    I64(Type),
    F32(Type),
    F64(Type),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    Packed,
    Planar,
}

impl SampleFormat {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(av_get_sample_fmt_name((*self).into())).to_bytes())
        }
    }

    #[inline]
    pub fn sample_type(&self) -> Option<SampleType> {
        match self {
            SampleFormat::None => None,
            SampleFormat::U8(_) => Some(SampleType::UInt8),
            SampleFormat::I16(_) => Some(SampleType::Int16),
            SampleFormat::I32(_) => Some(SampleType::Int32),
            SampleFormat::I64(_) => Some(SampleType::Int64),
            SampleFormat::F32(_) => Some(SampleType::Float),
            SampleFormat::F64(_) => Some(SampleType::Double),
        }
    }

    #[inline]
    pub fn packed(&self) -> Self {
        unsafe { SampleFormat::from(av_get_packed_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn planar(&self) -> Self {
        unsafe { SampleFormat::from(av_get_planar_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        unsafe { av_sample_fmt_is_planar((*self).into()) == 1 }
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        !self.is_planar()
    }

    #[inline]
    pub fn bytes(&self) -> usize {
        unsafe { av_get_bytes_per_sample((*self).into()) as usize }
    }

    pub fn from_type<T, P>() -> Self
        where T: Sample, P: SamplePackingType
    {
        let mut ret = match T::get_sample_type() {
            SampleType::Float => AV_SAMPLE_FMT_FLT,
            SampleType::Double => AV_SAMPLE_FMT_DBL,
            SampleType::UInt8 => AV_SAMPLE_FMT_U8,
            SampleType::Int16 => AV_SAMPLE_FMT_S16,
            SampleType::Int32 => AV_SAMPLE_FMT_S32,
            SampleType::Int64 => AV_SAMPLE_FMT_S64
        };

        ret = match P::get_packing_type() {
            SamplePacking::Deinterleaved => unsafe { av_get_planar_sample_fmt(ret) },
            SamplePacking::Interleaved => ret,
        };

        Self::from(ret)
    }

    pub fn from_storage<T, C, L, P, S>(s: &S) -> Self
        where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
    {
        SampleFormat::from_type::<T, P>()
    }
}

impl From<AVSampleFormat> for SampleFormat {
    #[inline]
    fn from(value: AVSampleFormat) -> Self {
        match value {
            AV_SAMPLE_FMT_NONE => SampleFormat::None,

            AV_SAMPLE_FMT_U8 => SampleFormat::U8(Type::Packed),
            AV_SAMPLE_FMT_S16 => SampleFormat::I16(Type::Packed),
            AV_SAMPLE_FMT_S32 => SampleFormat::I32(Type::Packed),
            AV_SAMPLE_FMT_S64 => SampleFormat::I64(Type::Packed),
            AV_SAMPLE_FMT_FLT => SampleFormat::F32(Type::Packed),
            AV_SAMPLE_FMT_DBL => SampleFormat::F64(Type::Packed),

            AV_SAMPLE_FMT_U8P => SampleFormat::U8(Type::Planar),
            AV_SAMPLE_FMT_S16P => SampleFormat::I16(Type::Planar),
            AV_SAMPLE_FMT_S32P => SampleFormat::I32(Type::Planar),
            AV_SAMPLE_FMT_S64P => SampleFormat::I64(Type::Planar),
            AV_SAMPLE_FMT_FLTP => SampleFormat::F32(Type::Planar),
            AV_SAMPLE_FMT_DBLP => SampleFormat::F64(Type::Planar),

            AV_SAMPLE_FMT_NB => SampleFormat::None,
        }
    }
}

impl From<&'static str> for SampleFormat {
    #[inline]
    fn from(value: &'static str) -> Self {
        unsafe {
            let value = CString::new(value).unwrap();

            SampleFormat::from(av_get_sample_fmt(value.as_ptr()))
        }
    }
}

impl Into<AVSampleFormat> for SampleFormat {
    #[inline]
    fn into(self) -> AVSampleFormat {
        match self {
            SampleFormat::None => AV_SAMPLE_FMT_NONE,

            SampleFormat::U8(Type::Packed) => AV_SAMPLE_FMT_U8,
            SampleFormat::I16(Type::Packed) => AV_SAMPLE_FMT_S16,
            SampleFormat::I32(Type::Packed) => AV_SAMPLE_FMT_S32,
            SampleFormat::I64(Type::Packed) => AV_SAMPLE_FMT_S64,
            SampleFormat::F32(Type::Packed) => AV_SAMPLE_FMT_FLT,
            SampleFormat::F64(Type::Packed) => AV_SAMPLE_FMT_DBL,

            SampleFormat::U8(Type::Planar) => AV_SAMPLE_FMT_U8P,
            SampleFormat::I16(Type::Planar) => AV_SAMPLE_FMT_S16P,
            SampleFormat::I32(Type::Planar) => AV_SAMPLE_FMT_S32P,
            SampleFormat::I64(Type::Planar) => AV_SAMPLE_FMT_S64P,
            SampleFormat::F32(Type::Planar) => AV_SAMPLE_FMT_FLTP,
            SampleFormat::F64(Type::Planar) => AV_SAMPLE_FMT_DBLP,
        }
    }
}

pub struct FormatIter {
    ptr: *const AVSampleFormat,
}

impl FormatIter {
    pub fn new(ptr: *const AVSampleFormat) -> Self {
        FormatIter { ptr: ptr }
    }
}

impl Iterator for FormatIter {
    type Item = SampleFormat;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == AVSampleFormat::AV_SAMPLE_FMT_NONE {
                return None;
            }

            let format = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(format)
        }
    }
}