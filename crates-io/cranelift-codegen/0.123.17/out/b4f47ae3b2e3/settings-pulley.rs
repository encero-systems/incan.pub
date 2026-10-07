#[derive(Clone, PartialEq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:426
/// Flags group `pulley`.
pub struct Flags {
    bytes: [u8; 2], // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:429
}
impl Flags {
    /// Create flags pulley settings group.
    #[allow(unused_variables, reason = "generated code")] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:26
    pub fn new(shared: &settings::Flags, builder: &Builder) -> Self {
        let bvec = builder.state_for("pulley"); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:31
        let mut pulley = Self { bytes: [0; 2] }; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:32
        debug_assert_eq!(bvec.len(), 2); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:38
        pulley.bytes[0..2].copy_from_slice(&bvec); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:43
        pulley // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:64
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> + use<> {
        let mut bytes = [0; 2]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:74
        bytes.copy_from_slice(&self.bytes[0..2]); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:75
        DESCRIPTORS.iter().filter_map(move |d| {
            let values = match &d.detail {
                detail::Detail::Preset => return None, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:78
                detail::Detail::Enum { last, enumerators } => Some(TEMPLATE.enums(*last, *enumerators)), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:79
                _ => None // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:80
            }
            ; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:82
            Some(Value { name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] }) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:83
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:85
    }
}
/// Values for `pulley.pointer_width`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:147
pub enum PointerWidth {
    /// `pointer32`.
    Pointer32, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
    /// `pointer64`.
    Pointer64, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:151
}
impl PointerWidth {
    /// Returns a slice with all possible [PointerWidth] values. // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:92
    pub fn all() -> &'static [PointerWidth] {
        &[ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:98
            Self::Pointer32, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
            Self::Pointer64, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:101
        ] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:104
    }
}
impl fmt::Display for PointerWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Pointer32 => "pointer32", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
            Self::Pointer64 => "pointer64", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:116
        }
        ) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:119
    }
}
impl core::str::FromStr for PointerWidth {
    type Err = (); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:125
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pointer32" => Ok(Self::Pointer32), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            "pointer64" => Ok(Self::Pointer64), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:129
            _ => Err(()), // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:131
        }
    }
}
/// User-defined settings.
#[allow(dead_code, reason = "generated code")] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:209
impl Flags {
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[1 + p / 8] & (1 << (p % 8)) != 0 // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:214
    }
    /// The width of pointers for this Pulley target
    /// Supported values:
    /// * 'pointer32'
    /// * 'pointer64'
    pub fn pointer_width(&self) -> PointerWidth {
        match self.bytes[0] {
            0 => {
                PointerWidth::Pointer32
            }
            1 => {
                PointerWidth::Pointer64
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Whether this is a big-endian target
    /// Whether this is a big-endian target
    pub fn big_endian(&self) -> bool {
        self.numbered_predicate(0) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
}
static DESCRIPTORS: [detail::Descriptor; 2] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:253
    detail::Descriptor {
        name: "pointer_width", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "The width of pointers for this Pulley target", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Enum { last: 1, enumerators: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:274
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "big_endian", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Whether this is a big-endian target", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 1, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:304
static ENUMERATORS: [&str; 2] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:307
    "pointer32", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
    "pointer64", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:310
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:313
static HASH_TABLE: [u16; 4] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:323
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    1, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:339
static PRESETS: [(u8, u8); 0] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:342
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:359
static TEMPLATE: detail::Template = detail::Template {
    name: "pulley", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:374
    descriptors: &DESCRIPTORS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:375
    enumerators: &ENUMERATORS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:376
    hash_table: &HASH_TABLE, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:377
    defaults: &[0x00, 0x00], // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:378
    presets: &PRESETS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:379
}
; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:382
/// Create a `settings::Builder` for the pulley settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:389
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[pulley]")?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:398
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:401
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:402
                writeln!(f)?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:406
            }
        }
        Ok(()) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:409
    }
}
impl Flags {
    /// Get the flag values as raw bytes for hashing.
    pub fn hash_key(&self) -> &[u8] {
        &self.bytes // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:419
    }
}
