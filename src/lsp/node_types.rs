//! Tree-sitter `node-types.json` parser.
//!
//! Deserializes the JSON schema of AST node types and classifies them
//! into product types (with fields), sum types (with subtypes), and
//! leaf types (terminals).

use serde_json::Value;
use std::collections::BTreeMap;

/// A field on a product-type node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeField {
    /// Field name (e.g., "name", "body", "parameters").
    pub name: String,
    /// Whether this field can contain multiple children.
    pub multiple: bool,
    /// Whether this field is required.
    pub required: bool,
    /// Possible types for this field (named types only).
    pub types: Vec<String>,
}

/// A node type from `node-types.json`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    /// Product type — a node with named fields.
    Product {
        name: String,
        fields: Vec<NodeField>,
        /// Anonymous children (not in named fields).
        children: Vec<String>,
    },
    /// Sum type — a supertype with alternatives.
    Sum { name: String, subtypes: Vec<String> },
    /// Leaf type — a terminal with no children.
    Leaf { name: String },
}

impl NodeType {
    /// The type name, regardless of variant.
    pub fn name(&self) -> &str {
        match self {
            NodeType::Product { name, .. } => name,
            NodeType::Sum { name, .. } => name,
            NodeType::Leaf { name } => name,
        }
    }

    /// Strip leading underscore from hidden supertypes.
    pub fn display_name(&self) -> &str {
        let n = self.name();
        n.strip_prefix('_').unwrap_or(n)
    }
}

/// Parse `node-types.json` content into a list of `NodeType`s.
///
/// Filters: only `"named": true` entries are included.
/// Anonymous tokens (operators, punctuation, keywords) are skipped.
pub fn parse_node_types(json: &str) -> Result<Vec<NodeType>, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON parse: {}", e))?;
    let array = value
        .as_array()
        .ok_or_else(|| "node-types.json: expected top-level array".to_string())?;

    let mut result = Vec::new();
    for entry in array {
        let named = entry
            .get("named")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !named {
            continue;
        }
        let type_name = entry
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "node-types.json: entry missing 'type' field".to_string())?
            .to_string();

        // Sum type: has "subtypes" array.
        if let Some(subtypes_val) = entry.get("subtypes") {
            let subtypes = parse_type_refs(subtypes_val);
            if !subtypes.is_empty() {
                result.push(NodeType::Sum {
                    name: type_name,
                    subtypes,
                });
                continue;
            }
        }

        // Product type: has "fields" object with at least one field.
        let fields = parse_fields(entry.get("fields"));
        let children = entry
            .get("children")
            .map(parse_child_types)
            .unwrap_or_default();

        if !fields.is_empty() {
            result.push(NodeType::Product {
                name: type_name,
                fields,
                children,
            });
        } else if !children.is_empty() {
            // Has children but no named fields — treat as product with
            // a single "children" field.
            result.push(NodeType::Product {
                name: type_name,
                fields: vec![NodeField {
                    name: "children".into(),
                    multiple: true,
                    required: !children.is_empty(),
                    types: children.clone(),
                }],
                children,
            });
        } else {
            // Leaf node.
            result.push(NodeType::Leaf { name: type_name });
        }
    }

    Ok(result)
}

/// Extract named type references from a subtypes or types array.
fn parse_type_refs(value: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(arr) = value.as_array() {
        for item in arr {
            let named = item.get("named").and_then(|v| v.as_bool()).unwrap_or(false);
            if named {
                if let Some(t) = item.get("type").and_then(|v| v.as_str()) {
                    refs.push(t.to_string());
                }
            }
        }
    }
    refs
}

/// Parse named child types from a children object.
fn parse_child_types(value: &Value) -> Vec<String> {
    if let Some(types_val) = value.get("types") {
        return parse_type_refs(types_val);
    }
    Vec::new()
}

/// Parse the "fields" object into a sorted list of `NodeField`s.
fn parse_fields(value: Option<&Value>) -> Vec<NodeField> {
    let obj = match value.and_then(|v| v.as_object()) {
        Some(o) => o,
        None => return Vec::new(),
    };

    // Use BTreeMap for deterministic ordering.
    let mut fields_map: BTreeMap<String, NodeField> = BTreeMap::new();

    for (field_name, field_val) in obj {
        let multiple = field_val
            .get("multiple")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let required = field_val
            .get("required")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let types = field_val
            .get("types")
            .map(parse_type_refs)
            .unwrap_or_default();

        // Skip fields with no named types (pure anonymous token fields).
        if types.is_empty() {
            continue;
        }

        fields_map.insert(
            field_name.clone(),
            NodeField {
                name: field_name.clone(),
                multiple,
                required,
                types,
            },
        );
    }

    fields_map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_array() {
        let types = parse_node_types("[]").unwrap();
        assert!(types.is_empty());
    }

    #[test]
    fn parse_invalid_json() {
        assert!(parse_node_types("not json").is_err());
    }

    #[test]
    fn parse_not_array() {
        assert!(parse_node_types("{}").is_err());
    }

    #[test]
    fn parse_unnamed_skipped() {
        let json = r#"[{"type": "+", "named": false}]"#;
        let types = parse_node_types(json).unwrap();
        assert!(types.is_empty());
    }

    #[test]
    fn parse_leaf() {
        let json = r#"[{"type": "identifier", "named": true}]"#;
        let types = parse_node_types(json).unwrap();
        assert_eq!(types.len(), 1);
        assert!(matches!(&types[0], NodeType::Leaf { name } if name == "identifier"));
    }

    #[test]
    fn parse_sum_type() {
        let json = r#"[{
            "type": "_expression",
            "named": true,
            "subtypes": [
                {"type": "call", "named": true},
                {"type": "identifier", "named": true}
            ]
        }]"#;
        let types = parse_node_types(json).unwrap();
        assert_eq!(types.len(), 1);
        match &types[0] {
            NodeType::Sum { name, subtypes } => {
                assert_eq!(name, "_expression");
                assert_eq!(subtypes, &["call", "identifier"]);
            }
            other => panic!("expected Sum, got {:?}", other),
        }
    }

    #[test]
    fn parse_product_type() {
        let json = r#"[{
            "type": "function_definition",
            "named": true,
            "fields": {
                "name": {
                    "multiple": false,
                    "required": true,
                    "types": [{"type": "identifier", "named": true}]
                },
                "body": {
                    "multiple": false,
                    "required": true,
                    "types": [{"type": "block", "named": true}]
                }
            }
        }]"#;
        let types = parse_node_types(json).unwrap();
        assert_eq!(types.len(), 1);
        match &types[0] {
            NodeType::Product { name, fields, .. } => {
                assert_eq!(name, "function_definition");
                assert_eq!(fields.len(), 2);
                // BTreeMap ordering: body before name.
                assert_eq!(fields[0].name, "body");
                assert_eq!(fields[1].name, "name");
            }
            other => panic!("expected Product, got {:?}", other),
        }
    }

    #[test]
    fn parse_optional_field() {
        let json = r#"[{
            "type": "function_definition",
            "named": true,
            "fields": {
                "return_type": {
                    "multiple": false,
                    "required": false,
                    "types": [{"type": "type", "named": true}]
                }
            }
        }]"#;
        let types = parse_node_types(json).unwrap();
        match &types[0] {
            NodeType::Product { fields, .. } => {
                assert!(!fields[0].required);
            }
            other => panic!("expected Product, got {:?}", other),
        }
    }

    #[test]
    fn parse_multiple_field() {
        let json = r#"[{
            "type": "block",
            "named": true,
            "fields": {
                "statements": {
                    "multiple": true,
                    "required": false,
                    "types": [{"type": "statement", "named": true}]
                }
            }
        }]"#;
        let types = parse_node_types(json).unwrap();
        match &types[0] {
            NodeType::Product { fields, .. } => {
                assert!(fields[0].multiple);
            }
            other => panic!("expected Product, got {:?}", other),
        }
    }

    #[test]
    fn parse_children_only_node() {
        let json = r#"[{
            "type": "argument_list",
            "named": true,
            "fields": {},
            "children": {
                "multiple": true,
                "required": false,
                "types": [
                    {"type": "expression", "named": true},
                    {"type": "keyword_argument", "named": true}
                ]
            }
        }]"#;
        let types = parse_node_types(json).unwrap();
        assert_eq!(types.len(), 1);
        match &types[0] {
            NodeType::Product {
                name,
                fields,
                children,
            } => {
                assert_eq!(name, "argument_list");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].name, "children");
                assert_eq!(children.len(), 2);
            }
            other => panic!("expected Product, got {:?}", other),
        }
    }

    #[test]
    fn parse_anonymous_subtypes_filtered() {
        let json = r#"[{
            "type": "_expression",
            "named": true,
            "subtypes": [
                {"type": "call", "named": true},
                {"type": "+", "named": false}
            ]
        }]"#;
        let types = parse_node_types(json).unwrap();
        match &types[0] {
            NodeType::Sum { subtypes, .. } => {
                assert_eq!(subtypes, &["call"]);
            }
            other => panic!("expected Sum, got {:?}", other),
        }
    }

    #[test]
    fn display_name_strips_underscore() {
        let node = NodeType::Sum {
            name: "_expression".into(),
            subtypes: vec![],
        };
        assert_eq!(node.display_name(), "expression");
    }

    #[test]
    fn display_name_no_underscore() {
        let node = NodeType::Leaf {
            name: "identifier".into(),
        };
        assert_eq!(node.display_name(), "identifier");
    }

    #[test]
    fn parse_fields_with_anonymous_only_types_skipped() {
        let json = r#"[{
            "type": "binary_expression",
            "named": true,
            "fields": {
                "operator": {
                    "multiple": false,
                    "required": true,
                    "types": [
                        {"type": "+", "named": false},
                        {"type": "-", "named": false}
                    ]
                },
                "left": {
                    "multiple": false,
                    "required": true,
                    "types": [{"type": "expression", "named": true}]
                }
            }
        }]"#;
        let types = parse_node_types(json).unwrap();
        match &types[0] {
            NodeType::Product { fields, .. } => {
                // "operator" field should be skipped (only anonymous types).
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].name, "left");
            }
            other => panic!("expected Product, got {:?}", other),
        }
    }

    #[test]
    fn parse_missing_type_field_errors() {
        let json = r#"[{"named": true}]"#;
        assert!(parse_node_types(json).is_err());
    }

    #[test]
    fn parse_python_fixture() {
        let fixture = include_str!("../../fixtures/node-types/python.json");
        let types = parse_node_types(fixture).unwrap();
        // Python has ~129 named types.
        assert!(types.len() > 100, "got {} types", types.len());

        // function_definition should be a product type.
        let func = types
            .iter()
            .find(|t| t.name() == "function_definition")
            .expect("function_definition");
        match func {
            NodeType::Product { fields, .. } => {
                let field_names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
                assert!(field_names.contains(&"name"));
                assert!(field_names.contains(&"body"));
                assert!(field_names.contains(&"parameters"));
            }
            other => panic!("expected Product, got {:?}", other),
        }

        // _expression should be a sum type.
        let has_expression = types
            .iter()
            .any(|t| matches!(t, NodeType::Sum { name, .. } if name.contains("expression")));
        assert!(has_expression);

        // identifier should be a leaf.
        let ident = types.iter().find(|t| t.name() == "identifier");
        assert!(matches!(ident, Some(NodeType::Leaf { .. })));
    }
}
