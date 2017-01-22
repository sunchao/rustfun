// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Input {
    // message fields
    op1: ::std::option::Option<i64>,
    op2: ::std::option::Option<i64>,
    op: ::std::option::Option<Input_Op>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Input {}

impl Input {
    pub fn new() -> Input {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Input {
        static mut instance: ::protobuf::lazy::Lazy<Input> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Input,
        };
        unsafe {
            instance.get(Input::new)
        }
    }

    // required int64 op1 = 1;

    pub fn clear_op1(&mut self) {
        self.op1 = ::std::option::Option::None;
    }

    pub fn has_op1(&self) -> bool {
        self.op1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op1(&mut self, v: i64) {
        self.op1 = ::std::option::Option::Some(v);
    }

    pub fn get_op1(&self) -> i64 {
        self.op1.unwrap_or(0)
    }

    fn get_op1_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.op1
    }

    fn mut_op1_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.op1
    }

    // required int64 op2 = 2;

    pub fn clear_op2(&mut self) {
        self.op2 = ::std::option::Option::None;
    }

    pub fn has_op2(&self) -> bool {
        self.op2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op2(&mut self, v: i64) {
        self.op2 = ::std::option::Option::Some(v);
    }

    pub fn get_op2(&self) -> i64 {
        self.op2.unwrap_or(0)
    }

    fn get_op2_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.op2
    }

    fn mut_op2_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.op2
    }

    // required .Input.Op op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: Input_Op) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> Input_Op {
        self.op.unwrap_or(Input_Op::ADD)
    }

    fn get_op_for_reflect(&self) -> &::std::option::Option<Input_Op> {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::std::option::Option<Input_Op> {
        &mut self.op
    }
}

impl ::protobuf::Message for Input {
    fn is_initialized(&self) -> bool {
        if self.op1.is_none() {
            return false;
        };
        if self.op2.is_none() {
            return false;
        };
        if self.op.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.op1 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.op2 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.op = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.op1 {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.op2 {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.op {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.op1 {
            os.write_int64(1, v)?;
        };
        if let Some(v) = self.op2 {
            os.write_int64(2, v)?;
        };
        if let Some(v) = self.op {
            os.write_enum(3, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Input {
    fn new() -> Input {
        Input::new()
    }

    fn descriptor_static(_: ::std::option::Option<Input>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "op1",
                    Input::get_op1_for_reflect,
                    Input::mut_op1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "op2",
                    Input::get_op2_for_reflect,
                    Input::mut_op2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Input_Op>>(
                    "op",
                    Input::get_op_for_reflect,
                    Input::mut_op_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Input>(
                    "Input",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Input {
    fn clear(&mut self) {
        self.clear_op1();
        self.clear_op2();
        self.clear_op();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Input {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Input {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Input_Op {
    ADD = 0,
    SUB = 1,
    MUL = 2,
    DIV = 3,
}

impl ::protobuf::ProtobufEnum for Input_Op {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Input_Op> {
        match value {
            0 => ::std::option::Option::Some(Input_Op::ADD),
            1 => ::std::option::Option::Some(Input_Op::SUB),
            2 => ::std::option::Option::Some(Input_Op::MUL),
            3 => ::std::option::Option::Some(Input_Op::DIV),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Input_Op] = &[
            Input_Op::ADD,
            Input_Op::SUB,
            Input_Op::MUL,
            Input_Op::DIV,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Input_Op>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Input_Op", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Input_Op {
}

impl ::protobuf::reflect::ProtobufValue for Input_Op {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Output {
    // message fields
    out: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Output {}

impl Output {
    pub fn new() -> Output {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Output {
        static mut instance: ::protobuf::lazy::Lazy<Output> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Output,
        };
        unsafe {
            instance.get(Output::new)
        }
    }

    // required int64 out = 1;

    pub fn clear_out(&mut self) {
        self.out = ::std::option::Option::None;
    }

    pub fn has_out(&self) -> bool {
        self.out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_out(&mut self, v: i64) {
        self.out = ::std::option::Option::Some(v);
    }

    pub fn get_out(&self) -> i64 {
        self.out.unwrap_or(0)
    }

    fn get_out_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.out
    }

    fn mut_out_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.out
    }
}

impl ::protobuf::Message for Output {
    fn is_initialized(&self) -> bool {
        if self.out.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.out = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.out {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.out {
            os.write_int64(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Output {
    fn new() -> Output {
        Output::new()
    }

    fn descriptor_static(_: ::std::option::Option<Output>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "out",
                    Output::get_out_for_reflect,
                    Output::mut_out_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Output>(
                    "Output",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Output {
    fn clear(&mut self) {
        self.clear_out();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Output {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Output {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x63, 0x61, 0x6c, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x75, 0x0a, 0x05,
    0x49, 0x6e, 0x70, 0x75, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6f, 0x70, 0x31, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x03, 0x52, 0x03, 0x6f, 0x70, 0x31, 0x12, 0x10, 0x0a, 0x03, 0x6f, 0x70, 0x32, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x03, 0x52, 0x03, 0x6f, 0x70, 0x32, 0x12, 0x1e, 0x0a, 0x02, 0x6f, 0x70, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x4f, 0x70,
    0x3a, 0x03, 0x41, 0x44, 0x44, 0x52, 0x02, 0x6f, 0x70, 0x22, 0x28, 0x0a, 0x02, 0x4f, 0x70, 0x12,
    0x07, 0x0a, 0x03, 0x41, 0x44, 0x44, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x55, 0x42, 0x10,
    0x01, 0x12, 0x07, 0x0a, 0x03, 0x4d, 0x55, 0x4c, 0x10, 0x02, 0x12, 0x07, 0x0a, 0x03, 0x44, 0x49,
    0x56, 0x10, 0x03, 0x22, 0x1a, 0x0a, 0x06, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x12, 0x10, 0x0a,
    0x03, 0x6f, 0x75, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x52, 0x03, 0x6f, 0x75, 0x74, 0x32,
    0x2a, 0x0a, 0x0a, 0x43, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x1c, 0x0a,
    0x09, 0x43, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x12, 0x06, 0x2e, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x1a, 0x07, 0x2e, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x4a, 0x99, 0x05, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x03, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x11, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x04, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x04, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x05, 0x02, 0x0a, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x07, 0x09, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x04, 0x07, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x06, 0x0a, 0x0b, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x07, 0x0a, 0x0b, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x08, 0x0a, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x0b, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x0e,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x13, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x0b, 0x15, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x07, 0x12, 0x03, 0x0b, 0x20, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04,
    0x12, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x06, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x13, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x13, 0x21, 0x27,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
