#[cfg(target_os = "macos")]
#[nolink]
extern mod uuid {
    fn uuid_generate(out: UUID);
    fn uuid_generate_random(out: UUID);
    fn uuid_generate_time(out: UUID);

    fn uuid_parse(s: *u8, uuid: UUID) -> libc::c_int;

    fn uuid_unparse(uuid: UUID, out: *u8);
    fn uuid_unparse_lower(uuid: UUID, out: *u8);
    fn uuid_unparse_upper(uuid: UUID, out: *u8);
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "freebsd")]
extern mod uuid {
    fn uuid_generate(out: UUID);
    fn uuid_generate_random(out: UUID);
    fn uuid_generate_time(out: UUID);

    fn uuid_parse(s: *u8, uuid: UUID) -> libc::c_int;

    fn uuid_unparse(uuid: UUID, out: *u8);
    fn uuid_unparse_lower(uuid: UUID, out: *u8);
    fn uuid_unparse_upper(uuid: UUID, out: *u8);
}

/// a uuid value
pub struct UUID {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl UUID: cmp::Eq {
    pure fn eq(other: &UUID) -> bool {
        self.a == other.a &&
        self.b == other.b &&
        self.c == other.c &&
        self.d == other.d
    }

    pure fn ne(other: &UUID) -> bool {
        !self.eq(other)
    }
}

impl UUID: cmp::Ord {
    pure fn lt(other: &UUID) -> bool {
        if self.a < other.a { return true; }
        if other.a < self.a { return false; }
        if self.b < other.b { return true; }
        if other.b < self.b { return false; }
        if self.c < other.c { return true; }
        if other.c < self.c { return false; }
        self.d < other.d
    }
    pure fn le(other: &UUID) -> bool { !(*other).lt(&self) }
    pure fn ge(other: &UUID) -> bool { !self.lt(other) }
    pure fn gt(other: &UUID) -> bool { (*other).lt(&self) }
}

/// Create a new uuid
pub fn UUID() -> UUID {
    let uuid = UUID { a: 0u32, b: 0u32, c: 0u32, d: 0u32 };
    uuid::uuid_generate(uuid);
    uuid
}

/// Create a uuid from the current time and mac address
pub fn UUID_random() -> UUID {
    let uuid = UUID { a: 0u32, b: 0u32, c: 0u32, d: 0u32 };
    uuid::uuid_generate_random(uuid);
    uuid
}

/// Create a uuid from a random number generator
pub fn UUID_time() -> UUID {
    let uuid = UUID { a: 0u32, b: 0u32, c: 0u32, d: 0u32 };
    uuid::uuid_generate_time(uuid);
    uuid
}

/// Convert a uuid to a string
pub impl UUID: to_str::ToStr {
    fn to_str() -> ~str {
        let mut s = ~"";
        str::reserve(&mut s, 36u);
        unsafe { str::raw::set_len(&mut s, 36u); }
        do str::as_buf(s) |buf, _len| {
            uuid::uuid_unparse(self, buf);
        }
        s
    }
}

pub fn from_str(s: &str) -> Option<UUID> {
    assert s.len() == 36u;

    let uuid = UUID { a: 0u32, b: 0u32, c: 0u32, d: 0u32 }; 
    do str::as_buf(s) |buf, _len| {
        uuid::uuid_parse(buf, uuid);
    }

    Some(uuid)
}

/// Convert a string to a uuid
impl UUID: from_str::FromStr {
    static fn from_str(s: &str) -> Option<UUID> { from_str(s) }
}

#[cfg(test)]
mod test {
    pub use from_str::FromStr;

    #[test]
    fn test() {
        for uint::range(0u, 100000u) |_i| {
            let uuid = UUID();
            assert from_str(uuid.to_str()) == Some(uuid);

            let uuid = UUID_random();
            assert from_str(uuid.to_str()) == Some(uuid);

            let uuid = UUID_time();
            assert from_str(uuid.to_str()) == Some(uuid)
        }
    }
}
