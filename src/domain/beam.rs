//! The BEAM domain. Desired state for BEAM topology.
//!
//! `@beam` is not a read model — it's a specification.
//! The conversation describes what the BEAM *should* look like.
//! The Gleam runtime converges toward the specification.
//!
//! The conversation IS the gradient: current BEAM → desired BEAM.

use super::Context;

/// The BEAM context — desired state specification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Beam;

/// Desired-state vocabulary for BEAM topology.
///
/// Each variant is a specification, not an observation.
/// The runtime reads these and converges toward them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeamNode {
    /// A process that should exist in this desired state.
    Process { name: String, desired_state: String },
    /// A supervision strategy that should be active.
    Supervision { strategy: String },
    /// A module that should be loaded/available.
    Module { name: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Addressable, Context};
    use crate::witness::ContentAddressed;
    use fragmentation::encoding::Encode;

    // -- Context --

    #[test]
    fn beam_id() {
        assert_eq!(Beam::id(), "beam");
    }

    #[test]
    fn beam_is_context() {
        fn requires_context<C: Context>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_context::<Beam>(), "beam");
    }

    // -- ContentAddressed --

    #[test]
    fn beam_node_process_content_addressed() {
        let a = BeamNode::Process {
            name: "health".into(),
            desired_state: "critical".into(),
        };
        let b = BeamNode::Process {
            name: "health".into(),
            desired_state: "critical".into(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn beam_node_different_state_different_oid() {
        let a = BeamNode::Process {
            name: "health".into(),
            desired_state: "critical".into(),
        };
        let b = BeamNode::Process {
            name: "health".into(),
            desired_state: "nominal".into(),
        };
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn beam_node_supervision_content_addressed() {
        let a = BeamNode::Supervision {
            strategy: "one_for_one".into(),
        };
        let b = BeamNode::Supervision {
            strategy: "one_for_one".into(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn beam_node_module_content_addressed() {
        let a = BeamNode::Module {
            name: "Conversation.Runtime".into(),
        };
        let b = BeamNode::Module {
            name: "Conversation.Runtime".into(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn beam_node_different_variant_different_oid() {
        let process = BeamNode::Process {
            name: "x".into(),
            desired_state: "y".into(),
        };
        let module = BeamNode::Module { name: "x".into() };
        assert_ne!(process.content_oid(), module.content_oid());
    }

    // -- Addressable --

    #[test]
    fn beam_node_addressable_process() {
        let node = BeamNode::Process {
            name: "health".into(),
            desired_state: "critical".into(),
        };
        assert_eq!(node.node_name(), "health");
        assert_eq!(node.node_content(), Some("critical"));
    }

    #[test]
    fn beam_node_addressable_supervision() {
        let node = BeamNode::Supervision {
            strategy: "one_for_one".into(),
        };
        assert_eq!(node.node_name(), "one_for_one");
        assert_eq!(node.node_content(), None);
    }

    #[test]
    fn beam_node_addressable_module() {
        let node = BeamNode::Module {
            name: "Runtime".into(),
        };
        assert_eq!(node.node_name(), "Runtime");
        assert_eq!(node.node_content(), None);
    }

    // -- Encode --

    #[test]
    fn beam_node_encode_process() {
        let node = BeamNode::Process {
            name: "health".into(),
            desired_state: "critical".into(),
        };
        assert_eq!(node.encode(), b"process:health:critical");
    }

    #[test]
    fn beam_node_encode_supervision() {
        let node = BeamNode::Supervision {
            strategy: "one_for_one".into(),
        };
        assert_eq!(node.encode(), b"supervision:one_for_one");
    }

    #[test]
    fn beam_node_encode_module() {
        let node = BeamNode::Module {
            name: "Runtime".into(),
        };
        assert_eq!(node.encode(), b"module:Runtime");
    }
}
