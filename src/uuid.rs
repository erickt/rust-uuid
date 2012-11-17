#[cfg(target_os = "macos")]
#[nolink]
extern mod uuid {
    fn uuid_generate(out: *UUID);
    fn uuid_generate_random(out: *UUID);
    fn uuid_generate_time(out: *UUID);

    fn uuid_copy(dst: *UUID, src: *UUID);
    fn uuid_parse(s: *u8, uuid: *UUID) -> libc::c_int;
    pure fn uuid_compare(uuid1: *UUID, uuid2: *UUID) -> libc::c_int;
    pure fn uuid_unparse(uuid: *UUID, out: *u8);
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "freebsd")]
extern mod uuid {
    fn uuid_generate(out: *UUID);
    fn uuid_generate_random(out: *UUID);
    fn uuid_generate_time(out: *UUID);

    fn uuid_copy(dst: *UUID, src: *UUID);
    fn uuid_parse(s: *u8, uuid: *UUID) -> libc::c_int;
    pure fn uuid_compare(uuid1: *UUID, uuid2: *UUID) -> libc::c_int;
    pure fn uuid_unparse(uuid: *UUID, out: *u8);
}

/// a uuid value
pub struct UUID {
    v1: u64,
    v2: u64
}

impl UUID: cmp::Eq {
    pure fn eq(other: &UUID) -> bool {
        uuid::uuid_compare(ptr::addr_of(&self), ptr::addr_of(other)) == 0
    }

    pure fn ne(other: &UUID) -> bool {
        !self.eq(other)
    }
}

impl UUID: cmp::Ord {
    pure fn lt(other: &UUID) -> bool {
        uuid::uuid_compare(ptr::addr_of(&self), ptr::addr_of(other)) < 0
    }
    pure fn le(other: &UUID) -> bool { !(*other).lt(&self) }
    pure fn ge(other: &UUID) -> bool { !self.lt(other) }
    pure fn gt(other: &UUID) -> bool { (*other).lt(&self) }
}

impl UUID {
  static priv pure fn new() -> UUID {
    UUID { v1: 0, v2: 0 }
  }
}

/// Create a new uuid
pub fn UUID() -> UUID {
    let uuid = UUID::new();
    uuid::uuid_generate(ptr::addr_of(&uuid));
    uuid
}

/// Create a uuid from the current time and mac address
pub fn UUID_random() -> UUID {
    let uuid = UUID::new();
    uuid::uuid_generate_random(ptr::addr_of(&uuid));
    uuid
}

/// Create a uuid from a random number generator
pub fn UUID_time() -> UUID {
    let uuid = UUID::new();
    uuid::uuid_generate_time(ptr::addr_of(&uuid));
    uuid
}

/// Convert a uuid to a string
pub impl UUID: ToStr {
    pure fn to_str() -> ~str {
        let mut s = ~"";
        unsafe { str::reserve(&mut s, 36u); }
        do str::as_buf(s) |buf, _len| {
            uuid::uuid_unparse(ptr::addr_of(&self), buf);
        }
        unsafe { str::raw::set_len(&mut s, 36u); }
        move s
    }
}

pub fn from_str(s: &str) -> Option<UUID> {
    assert s.len() == 36u;

    let uuid = UUID::new();
    do str::as_buf(s) |buf, _len| {
        uuid::uuid_parse(buf, ptr::addr_of(&uuid));
    }

    Some(uuid)
}

pub fn clone(uuid: UUID) -> UUID {
    let clone = UUID::new();
    uuid::uuid_copy(ptr::addr_of(&clone), ptr::addr_of(&uuid));
    clone
}

/// Convert a string to a uuid
impl UUID: FromStr {
    static fn from_str(s: &str) -> Option<UUID> { from_str(s) }
}

#[cfg(test)]
mod test {
    pub use FromStr;

    #[test]
    fn test_compare() {
        let uuid = UUID();
        let uuid2 = clone(uuid);

        assert uuid == uuid;
        assert uuid == uuid2;
        assert uuid >= uuid2;
        assert uuid <= uuid2;

        let uuid = UUID { v1: 123, v2: 123 };
        let uuid2 = UUID { v1: 122, v2: 123 };

        assert uuid > uuid2;
        assert uuid2 < uuid;
    }

    #[test]
    fn test_string() {
        let uuid = UUID();
        assert from_str(uuid.to_str()) == Some(uuid);
    }

    #[test]
    fn test_generate() {
        let uuid0 = UUID { v1: 0, v2: 0 };
        let uuid1 = UUID();
        let uuid2 = UUID_random();
        let uuid3 = UUID_time();

        assert uuid0 < uuid1;
        assert uuid0 < uuid2;
        assert uuid0 < uuid3;

        assert uuid1 != uuid2;
        assert uuid2 != uuid3;
        assert uuid3 != uuid1;
    }
}
