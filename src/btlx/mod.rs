//! BTLx — the open timber-fabrication interchange format we serialise to.
//!
//! [`model`] holds the schema-mirrored data types; [`to_xml`] renders a
//! [`Btlx`](model::Btlx) document to a machine-agnostic `.btlx` string that
//! validates against `fixtures/schema/BTLx_2_3_1.xsd`.
//!
//! ```
//! use factory_btlx::btlx::{model::*, to_xml};
//!
//! let part = Part::new(3000.0, 160.0, 80.0)
//!     .designation("beam-1")
//!     .with_processings(vec![Processing::Drilling(
//!         Drilling::new("bolt-hole", 1, RefPlane::Global(3), 500.0, 80.0, 80.0, 12.0),
//!     )]);
//! let doc = Btlx::new(Project::new("demo", vec![part]));
//! let xml = to_xml(&doc).unwrap();
//! assert!(xml.contains("<Drilling"));
//! ```

pub mod model;

pub use model::{
    Btlx, Drilling, JackRafterCut, Lap, Mortise, Orientation, Part, Parts, Processing, Processings,
    Project, RefPlane, Tenon,
};

/// Render a BTLx document to an indented XML string, with the standard
/// declaration header.
///
/// Output uses the design2machine default namespace and no element prefixes, as
/// required by the schema's `elementFormDefault="qualified"`.
pub fn to_xml(doc: &Btlx) -> Result<String, quick_xml::se::SeError> {
    use serde::Serialize;
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    let mut ser = quick_xml::se::Serializer::new(&mut out);
    ser.indent(' ', 2);
    doc.serialize(ser)?;
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::model::*;
    use super::to_xml;

    fn sample_doc() -> Btlx {
        let part = Part::new(3000.0, 160.0, 80.0)
            .designation("beam-1")
            .with_processings(vec![
                Processing::JackRafterCut(JackRafterCut::new(
                    "jack-cut-start",
                    1,
                    RefPlane::Global(1),
                    Orientation::Start,
                    0.0,
                    45.0,
                )),
                Processing::Lap(Lap::new(
                    "halving",
                    2,
                    RefPlane::Global(3),
                    Orientation::End,
                    2800.0,
                    0.0,
                    160.0,
                    80.0,
                    40.0,
                )),
                Processing::Mortise(Mortise::new(
                    "mortise-1",
                    3,
                    RefPlane::Global(1),
                    1500.0,
                    40.0,
                    100.0,
                    40.0,
                    60.0,
                )),
                Processing::Tenon(Tenon::new(
                    "tenon-1",
                    4,
                    RefPlane::Global(2),
                    Orientation::Start,
                    0.0,
                    40.0,
                    50.0,
                    40.0,
                    40.0,
                )),
                Processing::Drilling(Drilling::new(
                    "bolt-hole",
                    5,
                    RefPlane::Global(3),
                    500.0,
                    80.0,
                    80.0,
                    12.0,
                )),
            ]);
        Btlx::new(Project::new("demo-project", vec![part]))
    }

    #[test]
    fn emits_well_formed_btlx() {
        let xml = to_xml(&sample_doc()).unwrap();
        // Root, namespace, version.
        assert!(xml.contains("<BTLx"));
        assert!(xml.contains("xmlns=\"https://www.design2machine.com\""));
        assert!(xml.contains("Version=\"2.3.1\""));
        // Project + part dimensions as attributes.
        assert!(xml.contains("<Project Name=\"demo-project\""));
        assert!(xml.contains("Length=\"3000\""));
        // All five processing types present.
        for tag in ["<JackRafterCut", "<Lap", "<Mortise", "<Tenon", "<Drilling"] {
            assert!(xml.contains(tag), "missing {tag}");
        }
        // Orientation serialises as start/end; identity as attributes.
        assert!(xml.contains("<Orientation>start</Orientation>"));
        assert!(xml.contains("<Orientation>end</Orientation>"));
        assert!(xml.contains("ReferencePlaneID=\"3\""));
        assert!(xml.contains("<Diameter>12</Diameter>"));
    }
}
