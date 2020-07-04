/* mu/type.rs */
#[derive(Debug)]
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

pub enum SysClass {
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

/** * immediate pointer layout: [d*].lllttTTT **/
#[derive(FromPrimitive)]
pub enum ImmediateClass {
    Char = 0,
    String = 1,
    Keyword = 2,
    Float = 3
}

const IMMEDIATE_STR_MAX: u64 = 7;

const T: Type = Type {
    bits: (('t' as u64) << 8)
          | (1 << 5)
          | ((ImmediateClass::Keyword as u64) << 3)
          | (Tag::Immediate as u64)
};

const NIL: Type = Type {
    bits: (((('l' as u64) << 16)
           | (('i' as u64) << 8)
           | (('n' as u64))) << 8)
           | (3 << 5)
           | ((ImmediateClass::Keyword as u64) << 3)
           | (Tag::Immediate as u64)
};

impl Type {
    pub fn tag(&self) -> Tag {
        let tag: std::option::Option<Tag> =
            num::FromPrimitive::from_u64(self.bits & 0x7);
        match tag {
            Some(_) => tag.unwrap(),
            None => panic!("Unknown tag")
        }
    }

    pub fn entag(base: u64, tag: Tag) -> Type {
        Type {
            bits: base | tag as u64
        }
    }

    pub fn immediate(data: u64, len: u8, tag: ImmediateClass) -> Type {
        Type {
            bits: (data << 8)
                | ((len as u64) << 5)
                | ((tag as u64) << 3)
                | ((Tag::Immediate as u64))
        }
    }

    pub fn immediate_data(&self) -> u64 {
        return self.bits >> 8;
    }

    pub fn immediate_size(&self) -> u64 {
        return self.bits >> 5;
    }
    
    pub fn immediate_class(&self) -> ImmediateClass {
        let tag: std::option::Option<ImmediateClass> =
            num::FromPrimitive::from_u64(self.bits >> 3);
        match tag {
            Some(_) => tag.unwrap(),
            None => panic!("Unknown tag")
        }
    }

    pub fn is_immediate(&self) -> bool {
        match self.tag() {
            Tag::Immediate => true,
            _ => false
        }
    }
  
    pub fn is_extended(&self) -> bool {
        match self.tag() {
            Tag::Extend => true,
            _ => false
        }
    }

    pub fn eq(&self, ptr: Type) -> bool {
        return self.bits == ptr.bits;
    }
}

/***
template <typename T>
static T* Untag(TagPtr ptr) {
    return reinterpret_cast<T*>(to_underlying(ptr) & ~0x7);
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
  static constexpr bool Null(TagPtr ptr) { return Eq(ptr, NIL); }
***/
