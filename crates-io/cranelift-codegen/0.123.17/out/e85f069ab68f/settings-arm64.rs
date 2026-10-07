#[derive(Clone, PartialEq, Hash)] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:426
/// Flags group `arm64`.
pub struct Flags {
    bytes: [u8; 1], // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:429
}
impl Flags {
    /// Create flags arm64 settings group.
    #[allow(unused_variables, reason = "generated code")] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:26
    pub fn new(shared: &settings::Flags, builder: &Builder) -> Self {
        let bvec = builder.state_for("arm64"); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:31
        let mut arm64 = Self { bytes: [0; 1] }; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:32
        debug_assert_eq!(bvec.len(), 1); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:38
        arm64.bytes[0..1].copy_from_slice(&bvec); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:43
        arm64 // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:64
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> + use<> {
        let mut bytes = [0; 1]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:74
        bytes.copy_from_slice(&self.bytes[0..1]); // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:75
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
/// User-defined settings.
#[allow(dead_code, reason = "generated code")] // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:209
impl Flags {
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[0 + p / 8] & (1 << (p % 8)) != 0 // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:214
    }
    /// Has Large System Extensions (FEAT_LSE) support.
    pub fn has_lse(&self) -> bool {
        self.numbered_predicate(0) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Has Pointer authentication (FEAT_PAuth) support; enables the use of non-HINT instructions, but does not have an effect on code generation by itself.
    pub fn has_pauth(&self) -> bool {
        self.numbered_predicate(1) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Use half-precision floating point (FEAT_FP16) instructions.
    pub fn has_fp16(&self) -> bool {
        self.numbered_predicate(2) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// If function return address signing is enabled, then apply it to all functions; does not have an effect on code generation by itself.
    pub fn sign_return_address_all(&self) -> bool {
        self.numbered_predicate(3) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Use pointer authentication instructions to sign function return addresses; HINT-space instructions using the A key are generated and simple functions that do not use the stack are not affected unless overridden by other settings.
    pub fn sign_return_address(&self) -> bool {
        self.numbered_predicate(4) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Use the B key with pointer authentication instructions instead of the default A key; does not have an effect on code generation by itself. Some platform ABIs may require this, for example.
    pub fn sign_return_address_with_bkey(&self) -> bool {
        self.numbered_predicate(5) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
    /// Use Branch Target Identification (FEAT_BTI) instructions.
    pub fn use_bti(&self) -> bool {
        self.numbered_predicate(6) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:171
    }
}
static DESCRIPTORS: [detail::Descriptor; 7] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:253
    detail::Descriptor {
        name: "has_lse", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Has Large System Extensions (FEAT_LSE) support.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 0 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "has_pauth", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Has Pointer authentication (FEAT_PAuth) support; enables the use of non-HINT instructions, but does not have an effect on code generation by itself.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 1 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "has_fp16", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Use half-precision floating point (FEAT_FP16) instructions.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 2 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "sign_return_address_all", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "If function return address signing is enabled, then apply it to all functions; does not have an effect on code generation by itself.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 3 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "sign_return_address", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Use pointer authentication instructions to sign function return addresses; HINT-space instructions using the A key are generated and simple functions that do not use the stack are not affected unless overridden by other settings.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 4 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "sign_return_address_with_bkey", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Use the B key with pointer authentication instructions instead of the default A key; does not have an effect on code generation by itself. Some platform ABIs may require this, for example.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 5 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
    detail::Descriptor {
        name: "use_bti", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:261
        description: "Use Branch Target Identification (FEAT_BTI) instructions.", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:262
        offset: 0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:263
        detail: detail::Detail::Bool { bit: 6 }, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:266
    }
    , // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:288
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:304
static ENUMERATORS: [&str; 0] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:307
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:313
static HASH_TABLE: [u16; 16] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:323
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    5, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    6, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
    1, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    2, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    4, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    3, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:327
    0xffff, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:335
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:339
static PRESETS: [(u8, u8); 0] = [ // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:342
]; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:359
static TEMPLATE: detail::Template = detail::Template {
    name: "arm64", // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:374
    descriptors: &DESCRIPTORS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:375
    enumerators: &ENUMERATORS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:376
    hash_table: &HASH_TABLE, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:377
    defaults: &[0x00], // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:378
    presets: &PRESETS, // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:379
}
; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:382
/// Create a `settings::Builder` for the arm64 settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE) // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:389
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[arm64]")?; // /oven/sha256-bb900043b55684b5e7a66d1dcab96e6bb32155038659a0d990cf31cda7666205/src/gen_settings.rs:398
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
