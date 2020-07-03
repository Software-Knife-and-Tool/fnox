/* mu/type.rs */
pub struct Type {
    bits: u64
}

#[derive(FromPrimitive)]
pub enum Tag {
    Address = 0,   /* machine address */
    Efixnum = 1,   /* even fixnum (62 bits) */
    Symbol = 2,    /* symbol/keyword */
    Function = 3,  /* function */
    Cons = 4,      /* cons */
    Ofixnum = 5,   /* odd fixnum (62 bits) */
    Immediate = 6, /* immediate */
    Extend = 7     /* extended */
}

enum SysClass {
    Byte,
    Char,
    Code,
    Cons,
    Environment,
    Exception,
    Ffi,
    Fixnum,
    Float,
    Function,
    Macro,
    Namespace,
    Null,
    Stream,
    String,
    Struct,
    Symbol,
    T,
    Thread,
    Vector,
    View
}

enum ImmediateClass {
    Char = 0,
    String = 1,
    Keyword = 2,
    Float = 3
}

const IMMEDIATE_STR_MAX: u32 = 7;

pub fn tag_of(ptr: Type) -> Tag {
    let element: std::option::Option<Tag> = num::FromPrimitive::from_u64(ptr.bits & 0x7);
    match element {
        Some(_) => element.unwrap(),
        None => panic!("Unknown tag")
    }
}

impl Type {
/***
    pub fn make1() -> Env {
        println!("making env, damnit");
        Env {
            stuff: 0
        }
    }
***/
}

/***
template <typename T>
static T* Untag(TagPtr ptr) {
    return reinterpret_cast<T*>(to_underlying(ptr) & ~0x7);
}

static constexpr TagPtr Entag(TagPtr ptr, TAG tag) {
    return static_cast<TagPtr>(to_underlying(ptr) | to_underlying(tag));
}

static TagPtr Entag(void* caddr, TAG tag) {
    return static_cast<TagPtr>(reinterpret_cast<u64_t>(caddr) |
                               to_underlying(tag));
}

static void* ToAddress(TagPtr ptr) {
    return reinterpret_cast<void*>(to_underlying(ptr));
}

static TagPtr FromU64(u64_t ptr) { return static_cast<TagPtr>(ptr); }

static constexpr u64_t ToU64(TagPtr ptr) { return to_underlying(ptr); }


/** * immediate pointer layout: [d*].lllttTTT **/
static constexpr TagPtr MakeImmediate(u64_t data, size_t len,
                                      IMMEDIATE_CLASS tag) {
    return static_cast<TagPtr>(((data << 8) | ((len & 0x7) << 5) |
                                ((static_cast<uint8_t>(tag) & 0x3) << 3) |
                                (static_cast<uint8_t>(TAG::IMMEDIATE) & 0x7)));
}

static constexpr u64_t ImmediateData(TagPtr ptr) {
    return static_cast<u64_t>(ptr) >> 8;
}

static constexpr size_t ImmediateSize(TagPtr ptr) {
    return static_cast<size_t>((static_cast<u64_t>(ptr) >> 5) & 0x7);
}

static constexpr IMMEDIATE_CLASS ImmediateClass(TagPtr ptr) {
    return static_cast<IMMEDIATE_CLASS>((static_cast<u64_t>(ptr) >> 3) &
                                        0x3);
}

static constexpr TagPtr T = static_cast<TagPtr>(
    ('t' << 8) | ((1 & 0x7) << 5) |
    ((static_cast<uint8_t>(IMMEDIATE_CLASS::KEYWORD) & 0x3) << 3) |
    (static_cast<uint8_t>(TAG::IMMEDIATE) & 0x7));

static constexpr TagPtr NIL = static_cast<TagPtr>(
    ((('l' << 16) | ('i' << 8) | 'n') << 8) | ((3 & 0x7) << 5) |
    ((static_cast<uint8_t>(IMMEDIATE_CLASS::KEYWORD) & 0x3) << 3) |
    (static_cast<uint8_t>(TAG::IMMEDIATE) & 0x7));


  static SYS_CLASS TypeOf(TagPtr);
  static bool IsClassSymbol(TagPtr);
  static TagPtr MapClassSymbol(SYS_CLASS);
  static SYS_CLASS MapSymbolClass(TagPtr);

  static constexpr TagPtr BoolOf(bool test) { return test ? T : NIL; }

  static constexpr bool IsImmediate(TagPtr ptr) {
    return TagOf(ptr) == TAG::IMMEDIATE;
  }
  
  static constexpr bool IsExtended(TagPtr ptr) {
    return TagOf(ptr) == TAG::EXTEND;
  }
  
  static constexpr bool Eq(TagPtr p0, TagPtr p1) { return p0 == p1; }
  static constexpr bool Null(TagPtr ptr) { return Eq(ptr, NIL); }
***/
