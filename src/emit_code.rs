//! emit_code — generic code emitter driven by @code grammar templates.

#[derive(Clone, Debug)]
pub enum IoList {
    Chunk(Vec<u8>),
    Nested(Vec<IoList>),
    Empty,
}

impl IoList {
    pub fn text(s: &str) -> Self {
        IoList::Chunk(s.as_bytes().to_vec())
    }

    pub fn join(parts: Vec<IoList>) -> Self {
        IoList::Nested(parts)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.flatten_into(&mut out);
        out
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.to_bytes()).into_owned()
    }

    fn flatten_into(&self, out: &mut Vec<u8>) {
        match self {
            IoList::Chunk(bytes) => out.extend_from_slice(bytes),
            IoList::Nested(children) => {
                for child in children {
                    child.flatten_into(out);
                }
            }
            IoList::Empty => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iolist_empty() {
        let empty: Vec<u8> = vec![];
        assert_eq!(IoList::Empty.to_bytes(), empty);
    }

    #[test]
    fn iolist_chunk() {
        assert_eq!(IoList::text("hello").to_bytes(), b"hello".to_vec());
    }

    #[test]
    fn iolist_nested() {
        let list = IoList::join(vec![
            IoList::text("pub "),
            IoList::text("struct "),
            IoList::text("Foo;\n"),
        ]);
        assert_eq!(list.to_bytes(), b"pub struct Foo;\n".to_vec());
    }

    #[test]
    fn iolist_deep() {
        let inner = IoList::join(vec![IoList::text("a"), IoList::text("b")]);
        let outer = IoList::join(vec![inner, IoList::text("c")]);
        assert_eq!(outer.to_bytes(), b"abc".to_vec());
    }

    #[test]
    fn iolist_to_string() {
        let list = IoList::join(vec![IoList::text("hello "), IoList::text("world")]);
        assert_eq!(list.to_string_lossy(), "hello world");
    }

    #[test]
    fn iolist_empty_nested() {
        let list = IoList::join(vec![IoList::Empty, IoList::text("x"), IoList::Empty]);
        assert_eq!(list.to_bytes(), b"x".to_vec());
    }
}
