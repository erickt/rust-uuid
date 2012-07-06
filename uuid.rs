export uuid;
export uuid_random;
export uuid_time;
export to_str;
export from_str;

#[doc = "a uuid value"]
type uuid = {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
};

#[cfg(target_os = "macos")]
#[nolink]
extern mod uuid {
    fn uuid_generate(out: uuid);
    fn uuid_generate_random(out: uuid);
    fn uuid_generate_time(out: uuid);

    fn uuid_parse(s: *u8, uuid: uuid) -> libc::c_int;

    fn uuid_unparse(uuid: uuid, out: *u8);
    fn uuid_unparse_lower(uuid: uuid, out: *u8);
    fn uuid_unparse_upper(uuid: uuid, out: *u8);
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "freebsd")]
extern mod uuid {
    fn uuid_generate(out: uuid);
    fn uuid_generate_random(out: uuid);
    fn uuid_generate_time(out: uuid);

    fn uuid_parse(s: *u8, uuid: uuid) -> libc::c_int;

    fn uuid_unparse(uuid: uuid, out: *u8);
    fn uuid_unparse_lower(uuid: uuid, out: *u8);
    fn uuid_unparse_upper(uuid: uuid, out: *u8);
}

#[doc = "Create a new uuid"]
fn uuid() -> uuid {
    let uuid = { a: 0u32, b: 0u32, c: 0u32, d: 0u32 };
    uuid::uuid_generate(uuid);
    uuid
}

#[doc = "Create a uuid from the current time and mac address"]
fn uuid_random() -> uuid {
    let uuid = { a: 0u32, b: 0u32, c: 0u32, d: 0u32 };
    uuid::uuid_generate_random(uuid);
    uuid
}

#[doc = "Create a uuid from a random number generator"]
fn uuid_time() -> uuid {
    let uuid = { a: 0u32, b: 0u32, c: 0u32, d: 0u32 };
    uuid::uuid_generate_time(uuid);
    uuid
}

#[doc = "Convert a uuid to a string"]
fn to_str(uuid: uuid) -> str {
    let mut s = "";
    str::reserve(s, 36u);
    unsafe { str::unsafe::set_len(s, 36u); }
    do str::as_buf(s) |buf| {
        uuid::uuid_unparse(uuid, buf);
    }
    s
}

#[doc = "Convert a string to a uuid"]
fn from_str(s: str) -> uuid {
    assert str::len(s) == 36u;

    let uuid = { a: 0u32, b: 0u32, c: 0u32, d: 0u32 }; 
    do str::as_buf(s) |buf| {
        uuid::uuid_parse(buf, uuid);
    }
    uuid
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        for uint::range(0u, 100000u) |_i| {
            let uuid = uuid();
            assert uuid == from_str(to_str(uuid));

            let uuid = uuid_random();
            assert uuid == from_str(to_str(uuid));

            let uuid = uuid_time();
            assert uuid == from_str(to_str(uuid));
        }
    }
}
