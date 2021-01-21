use {
    indexmap::IndexMap,
    serde::{Deserialize, Serialize},
};

pub use serde_json::Number;

// Implementations of `Serialize` and `Deserialize` for declaration path
mod decl_path_serde;
// Implementations of `Serialize` and `Deserialize` for method requests and responses
mod method_req_res_serde;
// Implementations of `Serialize` and `Deserialize` for table member types
mod table_member_type_serde;

// Definition of an `Option` wrapper that is always `Some` for the purposes of ser/de.
mod ser_option;
pub use ser_option::SerOption;

// Definitions of span and source-location-related types.
mod span;
pub use span::{FileId, Span, Spanned};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DeclPath {
    pub library_name: String,
    pub decl_name: String,
}

pub type DeclMap = IndexMap<Spanned<DeclPath>, DeclType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: Spanned<String>,
    // FIXME(cramertj) the current IR doesn't include these, but it probably should?
    #[serde(skip_serializing, skip_deserializing)]
    pub attributes: Vec<Spanned<Attribute>>,
    #[serde(rename = "const_declarations")]
    pub consts: Vec<Spanned<Const>>,
    #[serde(rename = "bits_declarations")]
    pub bits: Vec<Spanned<Bits>>,
    #[serde(rename = "enum_declarations")]
    pub enums: Vec<Spanned<Enum>>,
    #[serde(rename = "interface_declarations")]
    pub protocols: Vec<Spanned<Protocol>>,
    #[serde(rename = "struct_declarations")]
    pub structs: Vec<Spanned<Struct>>,
    #[serde(rename = "table_declarations")]
    pub tables: Vec<Spanned<Table>>,
    #[serde(rename = "union_declarations")]
    pub unions: Vec<Spanned<Union>>,
    #[serde(rename = "xunion_declarations")]
    pub xunions: Vec<Spanned<XUnion>>,
    pub declaration_order: Vec<String>,
    pub declarations: DeclMap,
    pub library_dependencies: Vec<LibraryDep>,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub usings: Vec<Spanned<Using>>,
}

#[derive(Debug, Clone)]
pub enum Using {
    AliasOnly { name: Spanned<String>, r#type: Spanned<Type> },
    Import { name: Spanned<String>, alias: Option<Spanned<String>> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiteralKind {
    String,
    Numeric,
    True,
    False,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literal {
    pub kind: LiteralKind,
    #[serde(default)]
    pub value: Option<Spanned<String>>,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub unsanitized_value: Option<Spanned<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum Constant {
    Identifier { identifier: String },
    Literal { literal: Spanned<Literal> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Union {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<DeclPath>,
    pub members: Vec<Spanned<UnionMember>>,
    pub size: SerOption<u32>,
    pub alignment: SerOption<u32>,
    pub max_handles: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionMember {
    pub attributes: Vec<Spanned<Attribute>>,
    pub r#type: Spanned<Type>,
    pub name: Spanned<String>,
    pub offset: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUnion {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<DeclPath>,
    pub members: Vec<Spanned<XUnionMember>>,
    pub size: SerOption<u32>,
    pub alignment: SerOption<u32>,
    pub max_handles: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUnionMember {
    pub attributes: Vec<Spanned<Attribute>>,
    pub ordinal: SerOption<u64>,
    pub r#type: Spanned<Type>,
    pub name: Spanned<String>,
    pub offset: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<DeclPath>,
    pub members: Vec<Spanned<TableMember>>,
    pub size: SerOption<u32>,
    pub alignment: SerOption<u32>,
    pub max_handles: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMember {
    pub attributes: Vec<Spanned<Attribute>>,
    pub ordinal: SerOption<Spanned<Number>>,
    #[serde(flatten)]
    pub member_type: TableMemberType,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub unresolved_ordinal: Option<Spanned<Constant>>,
}

#[derive(Debug, Clone)]
pub enum TableMemberType {
    Reserved,
    Field {
        r#type: Spanned<Type>,
        name: Spanned<String>,
        maybe_default_value: Option<Spanned<Constant>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<DeclPath>,
    pub members: Vec<Spanned<StructMember>>,
    pub size: SerOption<u32>,
    pub alignment: SerOption<u32>,
    pub max_handles: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructMember {
    pub attributes: Vec<Spanned<Attribute>>,
    pub r#type: Spanned<Type>,
    pub name: Spanned<String>,
    pub offset: SerOption<u32>,
    pub maybe_default_value: Option<Spanned<Constant>>,
    pub max_handles: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "interface")]
pub struct Protocol {
    pub name: Spanned<DeclPath>,
    pub attributes: Vec<Spanned<Attribute>>, // maybe_attributes
    pub methods: Vec<Spanned<Method>>,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub unresolved_composed: Option<Vec<Spanned<ProtocolCompose>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Spanned<String>,
    pub value: SerOption<Spanned<String>>,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub unresolved_value: Option<Spanned<Literal>>,
}

#[derive(Debug, Clone)]
pub struct ProtocolCompose {
    pub name: Spanned<DeclPath>,
    pub methods: Option<Vec<Spanned<Method>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub attributes: Vec<Spanned<Attribute>>,
    pub ordinal: SerOption<u64>,
    pub generated_ordinal: SerOption<u64>,
    pub name: Spanned<String>,
    #[serde(flatten)]
    #[serde(with = "method_req_res_serde::request_ser")]
    pub request: Option<Spanned<MethodReqRes>>,
    #[serde(flatten)]
    #[serde(with = "method_req_res_serde::response_ser")]
    pub response: Option<Spanned<MethodReqRes>>,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub unresolved_response_error_type: Option<Spanned<Type>>,
}

// FIXME(cramertj, godtamit): This may need to be broken up into individual Request and Response
// types, as the JSON IR represents them vastly differently.
// Request types are defined in-line as a list of the parameters.
// Response types have multiple levels of indirection:
//  * A ProtocolName_MethodName_Result union containing:
//      * ProtocolName_MethodName_Response struct containing:
//          * Members are the list of response parameters
//      * The 'err' type (if defined)
#[derive(Debug, Clone)]
pub struct MethodReqRes {
    pub parameters: Vec<Spanned<Parameter>>,
    pub size: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub r#type: Spanned<Type>,
    pub name: Spanned<String>,
    pub offset: SerOption<u32>,
    pub max_handles: SerOption<u32>,
    pub max_out_of_line: SerOption<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub attributes: Vec<Spanned<Attribute>>,
    // FIXME(cramertj): ensure that this is expecting specifically a PrimitiveSubtype and not a Type
    pub r#type: SerOption<Spanned<PrimitiveSubtype>>,
    pub name: Spanned<DeclPath>,
    pub members: Vec<Spanned<EnumMember>>,
    // note: these are used only prior to resolution.
    #[serde(skip_serializing, skip_deserializing)]
    pub unresolved_type: Option<Spanned<Type>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumMember {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<String>,
    pub value: SerOption<Spanned<Constant>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bits {
    pub attributes: Vec<Spanned<Attribute>>,
    pub r#type: SerOption<Spanned<Type>>,
    pub name: Spanned<DeclPath>,
    pub members: Vec<Spanned<BitsMember>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitsMember {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<String>,
    pub value: SerOption<Spanned<Constant>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Const {
    pub attributes: Vec<Spanned<Attribute>>,
    pub r#type: Spanned<Type>,
    pub name: Spanned<DeclPath>,
    pub value: Spanned<Constant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    #[serde(flatten)]
    pub kind: Spanned<TypeKind>,
    #[serde(default)]
    pub nullable: Spanned<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HandleSubtype {
    Bti,
    Channel,
    DebugLog,
    Eventpair,
    Event,
    Exception,
    Handle,
    Interrupt,
    Iommu,
    Fifo,
    Guest,
    Job,
    Pager,
    PciDevice,
    Pmt,
    Port,
    Process,
    Profile,
    Resource,
    Socket,
    SuspendToken,
    Thread,
    Timer,
    VCpu,
    Vmar,
    Vmo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveSubtype {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum TypeKind {
    Array {
        element_type: Box<Spanned<Type>>,
        element_count: SerOption<Spanned<Number>>,
        // note: these are used only prior to resolution.
        #[serde(skip_serializing, skip_deserializing)]
        unresolved_element_count: Option<Spanned<Constant>>,
    },
    Vector {
        element_type: Box<Spanned<Type>>,
        #[serde(default)]
        maybe_element_count: Option<Spanned<Number>>,
        // note: these are used only prior to resolution.
        #[serde(skip_serializing, skip_deserializing)]
        unresolved_maybe_element_count: Option<Spanned<Constant>>,
    },
    String {
        #[serde(default)]
        maybe_element_count: Option<Spanned<Number>>,
        // note: these are used only prior to resolution.
        #[serde(skip_serializing, skip_deserializing)]
        unresolved_maybe_element_count: Option<Spanned<Constant>>,
    },
    Handle {
        subtype: HandleSubtype,
    },
    Request {
        subtype: DeclPath,
        // note: these are used only prior to resolution.
        #[serde(skip_serializing, skip_deserializing)]
        unresolved: Option<Box<Spanned<String>>>,
    },
    Primitive {
        subtype: PrimitiveSubtype,
    },
    Identifier {
        identifier: Spanned<DeclPath>,
        // note: these are used only prior to resolution.
        #[serde(skip_serializing, skip_deserializing)]
        unresolved: Option<Box<Spanned<String>>>,
    },
    /// These variants should only ever appear immediately after parsing,
    /// before resolution, where they should be transformed into their
    /// resolved equivalents.
    #[serde(skip_serializing, skip_deserializing)]
    UnresolvedRequest {
        unresolved: Box<Spanned<String>>,
    },
    #[serde(skip_serializing, skip_deserializing)]
    UnresolvedIdentifier {
        unresolved: Box<Spanned<String>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeclType {
    Const,
    Bits,
    Enum,
    #[serde(rename = "interface")]
    Protocol,
    Struct,
    Table,
    Union,
    XUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDep {
    pub name: String,
    pub declarations: SerOption<DeclMap>,
}
