//! Content addressing. Byte-for-byte equivalent to C `content_oid`.

use crate::ast::{AstKind, AstNode};
use crate::hash::hash_tagged;

pub fn content_oid(node: &AstNode) -> String {
    let mut buf: Vec<u8> = Vec::new();
    match node.kind {
        AstKind::Focus => {
            buf.extend_from_slice(node.name.as_bytes());
            for c in &node.children {
                let child = content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("focus", &buf)
        }
        AstKind::Project => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("project", &buf)
        }
        AstKind::Split => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("split", &buf)
        }
        AstKind::Zoom => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("zoom", &buf)
        }
        AstKind::Refract => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("refract", &buf)
        }
        AstKind::In => hash_tagged("in", node.name.as_bytes()),
        AstKind::Out => hash_tagged("out", node.name.as_bytes()),
    }
}
