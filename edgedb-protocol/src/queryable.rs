use snafu::{Snafu, ensure};

use crate::errors::{self, DecodeError};
use crate::descriptors::{Descriptor, TypePos};

pub trait Queryable: Sized {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError>;
    fn decode_optional(buf: Option<&[u8]>) -> Result<Self, DecodeError> {
        ensure!(buf.is_some(), errors::MissingRequiredElement);
        Self::decode(buf.unwrap())
    }
    fn check_descriptor(ctx: &DescriptorContext, type_pos: TypePos)
        -> Result<(), DescriptorMismatch>;
}

#[derive(Snafu, Debug)]
#[non_exhaustive]
pub enum DescriptorMismatch {
    #[snafu(display("unexpected type {}, expected {}", unexpected, expected))]
    WrongType { unexpected: String, expected: String },
    #[snafu(display("unexpected field {}, expected {}", unexpected, expected))]
    WrongField { unexpected: String, expected: String },
    #[snafu(display("expected {} fields, got {}", expected, unexpected))]
    FieldNumber { unexpected: usize, expected: usize },
    #[snafu(display("expected {}", expected))]
    Expected { expected: String },
    #[snafu(display("invalid type descriptor"))]
    InvalidDescriptor,
}

pub struct DescriptorContext<'a> {
    descriptors: &'a [Descriptor],
}

impl DescriptorContext<'_> {
    pub(crate) fn new(descriptors: &[Descriptor]) -> DescriptorContext {
        DescriptorContext { descriptors }
    }
    pub fn get(&self, type_pos: TypePos)
        -> Result<&Descriptor, DescriptorMismatch>
    {
        self.descriptors.get(type_pos.0 as usize)
            .ok_or(DescriptorMismatch::InvalidDescriptor)
    }
    pub fn wrong_type(&self, descriptor: &Descriptor, expected: &str)
        -> DescriptorMismatch
    {
        DescriptorMismatch::WrongType {
            // TODO(tailhook) human-readable type description
            unexpected: format!("{:?}", descriptor),
            expected: expected.into(),
        }
    }
    pub fn field_number(&self, expected: usize, unexpected: usize)
        -> DescriptorMismatch
    {
        DescriptorMismatch::FieldNumber { expected, unexpected }
    }
    pub fn wrong_field(&self, expected: &str, unexpected: &str)
        -> DescriptorMismatch
    {
        DescriptorMismatch::WrongField {
            expected: expected.into(),
            unexpected: unexpected.into(),
        }
    }
    pub fn expected(&self, expected: &str)
        -> DescriptorMismatch
    {
        DescriptorMismatch::Expected { expected: expected.into() }
    }
}
