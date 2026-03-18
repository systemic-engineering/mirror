//! C-FFI surface for the conversation parser.
//!
//! Exposes `conv_parse` as an `extern "C"` function callable from C NIF wrappers.
//! Uses write-to-buffer pattern — no heap allocation crosses the FFI boundary.

use crate::parse::Parse;
use crate::ContentAddressed;
use crate::Vector;

/// Parse a .conv source string and return its content OID.
///
/// On success: returns 0, writes OID hex to `out_ptr` (up to `out_cap` bytes),
///             sets `*out_len` to the number of bytes written.
/// On error:   returns -1, writes error message to `out_ptr`, sets `*out_len`.
///
/// # Safety
///
/// - `src_ptr` must point to `src_len` valid UTF-8 bytes.
/// - `out_ptr` must point to a buffer of at least `out_cap` bytes.
/// - `out_len` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn conv_parse(
    src_ptr: *const u8,
    src_len: usize,
    out_ptr: *mut u8,
    out_cap: usize,
    out_len: *mut usize,
) -> i32 {
    let source = match std::str::from_utf8(std::slice::from_raw_parts(src_ptr, src_len)) {
        Ok(s) => s,
        Err(_) => {
            let msg = b"invalid UTF-8 input";
            let n = msg.len().min(out_cap);
            std::ptr::copy_nonoverlapping(msg.as_ptr(), out_ptr, n);
            *out_len = n;
            return -1;
        }
    };

    match Parse.trace(source.to_string()).into_result() {
        Ok(tree) => {
            let oid = tree.content_oid();
            let oid_str = oid.as_ref().as_bytes();
            let n = oid_str.len().min(out_cap);
            std::ptr::copy_nonoverlapping(oid_str.as_ptr(), out_ptr, n);
            *out_len = n;
            0
        }
        Err(err) => {
            let msg = err.to_string();
            let msg_bytes = msg.as_bytes();
            let n = msg_bytes.len().min(out_cap);
            std::ptr::copy_nonoverlapping(msg_bytes.as_ptr(), out_ptr, n);
            *out_len = n;
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffi_parse_success() {
        let source = b"grammar @test {\n  type = a | b\n}\n";
        let mut buf = [0u8; 256];
        let mut len: usize = 0;

        let rc = unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };

        assert_eq!(rc, 0);
        assert!(len > 0);
        let oid = std::str::from_utf8(&buf[..len]).unwrap();
        assert!(!oid.is_empty());
    }

    #[test]
    fn ffi_parse_error() {
        let source = b"@@@invalid";
        let mut buf = [0u8; 256];
        let mut len: usize = 0;

        let rc = unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };

        assert_eq!(rc, -1);
        assert!(len > 0);
    }

    #[test]
    fn ffi_parse_invalid_utf8() {
        let source: &[u8] = &[0xFF, 0xFE, 0x00];
        let mut buf = [0u8; 256];
        let mut len: usize = 0;

        let rc = unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf.as_mut_ptr(),
                buf.len(),
                &mut len,
            )
        };

        assert_eq!(rc, -1);
        assert!(len > 0);
        let msg = std::str::from_utf8(&buf[..len]).unwrap();
        assert!(msg.contains("UTF-8"));
    }

    #[test]
    fn ffi_parse_deterministic() {
        let source = b"grammar @test {\n  type = a | b\n}\n";
        let mut buf1 = [0u8; 256];
        let mut len1: usize = 0;
        let mut buf2 = [0u8; 256];
        let mut len2: usize = 0;

        unsafe {
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf1.as_mut_ptr(),
                256,
                &mut len1,
            );
            conv_parse(
                source.as_ptr(),
                source.len(),
                buf2.as_mut_ptr(),
                256,
                &mut len2,
            );
        }

        assert_eq!(len1, len2);
        assert_eq!(&buf1[..len1], &buf2[..len2]);
    }
}
