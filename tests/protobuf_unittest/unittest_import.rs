// This file is generated by rust-protobuf 2.3.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
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
pub struct ImportMessage {
    // message fields
    d: ::std::option::Option<i32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ImportMessage {
    pub fn new() -> ImportMessage {
        ::std::default::Default::default()
    }

    // optional int32 d = 1;

    pub fn clear_d(&mut self) {
        self.d = ::std::option::Option::None;
    }

    pub fn has_d(&self) -> bool {
        self.d.is_some()
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: i32) {
        self.d = ::std::option::Option::Some(v);
    }

    pub fn get_d(&self) -> i32 {
        self.d.unwrap_or(0)
    }
}

impl ::protobuf::Message for ImportMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.d = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.d {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.d {
            os.write_int32(1, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ImportMessage {
        ImportMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "d",
                    |m: &ImportMessage| { &m.d },
                    |m: &mut ImportMessage| { &mut m.d },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImportMessage>(
                    "ImportMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ImportMessage {
        static mut instance: ::protobuf::lazy::Lazy<ImportMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImportMessage,
        };
        unsafe {
            instance.get(ImportMessage::new)
        }
    }
}

impl ::protobuf::Clear for ImportMessage {
    fn clear(&mut self) {
        self.clear_d();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImportMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ImportEnum {
    IMPORT_FOO = 7,
    IMPORT_BAR = 8,
    IMPORT_BAZ = 9,
}

impl ::protobuf::ProtobufEnum for ImportEnum {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ImportEnum> {
        match value {
            7 => ::std::option::Option::Some(ImportEnum::IMPORT_FOO),
            8 => ::std::option::Option::Some(ImportEnum::IMPORT_BAR),
            9 => ::std::option::Option::Some(ImportEnum::IMPORT_BAZ),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ImportEnum] = &[
            ImportEnum::IMPORT_FOO,
            ImportEnum::IMPORT_BAR,
            ImportEnum::IMPORT_BAZ,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ImportEnum", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ImportEnum {
}

// Note, `Default` is implemented although default value is not 0
impl ::std::default::Default for ImportEnum {
    fn default() -> Self {
        ImportEnum::IMPORT_FOO
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportEnum {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ImportEnumForMap {
    UNKNOWN = 0,
    FOO = 1,
    BAR = 2,
}

impl ::protobuf::ProtobufEnum for ImportEnumForMap {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ImportEnumForMap> {
        match value {
            0 => ::std::option::Option::Some(ImportEnumForMap::UNKNOWN),
            1 => ::std::option::Option::Some(ImportEnumForMap::FOO),
            2 => ::std::option::Option::Some(ImportEnumForMap::BAR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ImportEnumForMap] = &[
            ImportEnumForMap::UNKNOWN,
            ImportEnumForMap::FOO,
            ImportEnumForMap::BAR,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ImportEnumForMap", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ImportEnumForMap {
}

impl ::std::default::Default for ImportEnumForMap {
    fn default() -> Self {
        ImportEnumForMap::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportEnumForMap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%google/protobuf/unittest_import.proto\x12\x18protobuf_unittest_import\
    \x1a,google/protobuf/unittest_import_public.protoP\0\"\x1d\n\rImportMess\
    age\x12\x0c\n\x01d\x18\x01\x20\x01(\x05R\x01d*<\n\nImportEnum\x12\x0e\n\
    \nIMPORT_FOO\x10\x07\x12\x0e\n\nIMPORT_BAR\x10\x08\x12\x0e\n\nIMPORT_BAZ\
    \x10\t*1\n\x10ImportEnumForMap\x12\x0b\n\x07UNKNOWN\x10\0\x12\x07\n\x03F\
    OO\x10\x01\x12\x07\n\x03BAR\x10\x02B\x1f\n\x18com.google.protobuf.testH\
    \x01\xf8\x01\x01J\xc0\x15\n\x06\x12\x04$\0H\x01\n\x98\x0e\n\x01\x0c\x12\
    \x03$\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20data\x20\
    interchange\x20format\n\x20Copyright\x202008\x20Google\x20Inc.\x20\x20Al\
    l\x20rights\x20reserved.\n\x20https://developers.google.com/protocol-buf\
    fers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\x20and\x20bina\
    ry\x20forms,\x20with\x20or\x20without\n\x20modification,\x20are\x20permi\
    tted\x20provided\x20that\x20the\x20following\x20conditions\x20are\n\x20m\
    et:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\x20source\x20code\
    \x20must\x20retain\x20the\x20above\x20copyright\n\x20notice,\x20this\x20\
    list\x20of\x20conditions\x20and\x20the\x20following\x20disclaimer.\n\x20\
    \x20\x20\x20\x20*\x20Redistributions\x20in\x20binary\x20form\x20must\x20\
    reproduce\x20the\x20above\n\x20copyright\x20notice,\x20this\x20list\x20o\
    f\x20conditions\x20and\x20the\x20following\x20disclaimer\n\x20in\x20the\
    \x20documentation\x20and/or\x20other\x20materials\x20provided\x20with\
    \x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\x20Neither\x20the\x20n\
    ame\x20of\x20Google\x20Inc.\x20nor\x20the\x20names\x20of\x20its\n\x20con\
    tributors\x20may\x20be\x20used\x20to\x20endorse\x20or\x20promote\x20prod\
    ucts\x20derived\x20from\n\x20this\x20software\x20without\x20specific\x20\
    prior\x20written\x20permission.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDE\
    D\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\
    \x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INC\
    LUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIE\
    S\x20OF\x20MERCHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\
    \x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\
    \x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\
    \x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLAR\
    Y,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20L\
    IMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVI\
    CES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINES\
    S\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\
    \x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILIT\
    Y,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20AR\
    ISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20\
    SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20\
    SUCH\x20DAMAGE.\n2\xc9\x01\x20Author:\x20kenton@google.com\x20(Kenton\
    \x20Varda)\n\x20\x20Based\x20on\x20original\x20Protocol\x20Buffers\x20de\
    sign\x20by\n\x20\x20Sanjay\x20Ghemawat,\x20Jeff\x20Dean,\x20and\x20other\
    s.\n\n\x20A\x20proto\x20file\x20which\x20is\x20imported\x20by\x20unittes\
    t.proto\x20to\x20test\x20importing.\n\n\xf5\x01\n\x01\x02\x12\x03*\x08\
    \x20\x1a\xea\x01\x20We\x20don't\x20put\x20this\x20in\x20a\x20package\x20\
    within\x20proto2\x20because\x20we\x20need\x20to\x20make\x20sure\n\x20tha\
    t\x20the\x20generated\x20code\x20doesn't\x20depend\x20on\x20being\x20in\
    \x20the\x20proto2\x20namespace.\n\x20In\x20test_util.h\x20we\x20do\n\x20\
    \"using\x20namespace\x20unittest_import\x20=\x20protobuf_unittest_import\
    \".\n\n\x08\n\x01\x08\x12\x03,\0\x1c\n\t\n\x02\x08\t\x12\x03,\0\x1c\n\
    \x08\n\x01\x08\x12\x03-\0\x1f\n\t\n\x02\x08\x1f\x12\x03-\0\x1f\n\x08\n\
    \x01\x08\x12\x030\01\n.\n\x02\x08\x01\x12\x030\01\x1a#\x20Exercise\x20th\
    e\x20java_package\x20option.\n\n\t\n\x02\n\0\x12\x036\x07\r\ns\n\x02\x03\
    \0\x12\x036\x0e<\x1a\x14\x20Test\x20public\x20import\n2R\x20Do\x20not\
    \x20set\x20a\x20java_outer_classname\x20here\x20to\x20verify\x20that\x20\
    Proto2\x20works\x20without\n\x20one.\n\n\n\n\x02\x04\0\x12\x048\0:\x01\n\
    \n\n\x03\x04\0\x01\x12\x038\x08\x15\n\x0b\n\x04\x04\0\x02\0\x12\x039\x02\
    \x17\n\x0c\n\x05\x04\0\x02\0\x04\x12\x039\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x039\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x039\x11\x12\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x039\x15\x16\n\n\n\x02\x05\0\x12\x04<\0@\x01\
    \n\n\n\x03\x05\0\x01\x12\x03<\x05\x0f\n\x0b\n\x04\x05\0\x02\0\x12\x03=\
    \x02\x11\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03=\x02\x0c\n\x0c\n\x05\x05\0\
    \x02\0\x02\x12\x03=\x0f\x10\n\x0b\n\x04\x05\0\x02\x01\x12\x03>\x02\x11\n\
    \x0c\n\x05\x05\0\x02\x01\x01\x12\x03>\x02\x0c\n\x0c\n\x05\x05\0\x02\x01\
    \x02\x12\x03>\x0f\x10\n\x0b\n\x04\x05\0\x02\x02\x12\x03?\x02\x11\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03?\x02\x0c\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03?\x0f\x10\nH\n\x02\x05\x01\x12\x04D\0H\x01\x1a<\x20To\x20use\x20\
    an\x20enum\x20in\x20a\x20map,\x20it\x20must\x20has\x20the\x20first\x20va\
    lue\x20as\x200.\n\n\n\n\x03\x05\x01\x01\x12\x03D\x05\x15\n\x0b\n\x04\x05\
    \x01\x02\0\x12\x03E\x02\x0e\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03E\x02\t\
    \n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03E\x0c\r\n\x0b\n\x04\x05\x01\x02\
    \x01\x12\x03F\x02\n\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03F\x02\x05\n\
    \x0c\n\x05\x05\x01\x02\x01\x02\x12\x03F\x08\t\n\x0b\n\x04\x05\x01\x02\
    \x02\x12\x03G\x02\n\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03G\x02\x05\n\
    \x0c\n\x05\x05\x01\x02\x02\x02\x12\x03G\x08\t\
";

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
