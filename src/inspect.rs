//! Inspecting real `.btlx` files.
//!
//! A robust, forgiving pass over any BTLx file: it does *not* need a complete
//! typed model of the schema, so it works on files with processings we don't yet
//! serialise. It answers the field question "does our code understand this real
//! file, and what's in it?" — the version, the part count, and a histogram of
//! processing types, flagging any our serialiser can't yet produce.

use std::collections::BTreeMap;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Every processing type defined in the BTLx schema (types extending
/// `ProcessingType`, BTLx 2.3.1). Used to recognise processing elements in a file
/// regardless of whether we can serialise them yet.
pub const PROCESSING_TYPES: &[&str] = &[
    "BirdsMouth",
    "Connector",
    "CrampContour",
    "DoubleCut",
    "Dovetail",
    "DovetailMortise",
    "DovetailTenon",
    "Drilling",
    "FreeContour",
    "FreeSurface",
    "FrenchRidgeLap",
    "GlueArea",
    "HipValleyRafterNotch",
    "InsulationArea",
    "JackRafterCut",
    "JapaneseTenon",
    "Lap",
    "LockoutArea",
    "LogHouseFront",
    "LogHouseHalfLap",
    "LogHouseJoint",
    "LongitudinalCut",
    "Marking",
    "MillContour",
    "Mortise",
    "NailContour",
    "PatternContour",
    "PenContour",
    "PlaningArea",
    "PlasterArea",
    "Pocket",
    "ProfileCambered",
    "ProfileFront",
    "ProfileHead",
    "RidgeValleyCut",
    "RoundArch",
    "SawContour",
    "SawCut",
    "ScarfJoint",
    "ScrewContour",
    "SimpleScarf",
    "Slot",
    "Sphere",
    "StepJoint",
    "StepJointNotch",
    "Tenon",
    "Text",
    "TriangleCut",
    "TyroleanDovetail",
];

/// Processing types this crate's serialiser can currently *produce*. Everything
/// else in [`PROCESSING_TYPES`] we can read/recognise but not yet write.
pub const SERIALISABLE: &[&str] = &["Drilling"];

/// What an inspection found in a `.btlx` file.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    /// The `Version` attribute of the `<BTLx>` root, if present.
    pub version: Option<String>,
    /// The `Language` attribute, if present.
    pub language: Option<String>,
    /// Number of `<Part>` elements.
    pub parts: usize,
    /// Processing element name → count, across the whole file.
    pub processings: BTreeMap<String, usize>,
}

impl Report {
    /// Total processings across all parts.
    pub fn total_processings(&self) -> usize {
        self.processings.values().sum()
    }

    /// Processing types present in the file that our serialiser cannot yet write.
    pub fn unsupported(&self) -> Vec<&str> {
        self.processings
            .keys()
            .map(String::as_str)
            .filter(|k| !SERIALISABLE.contains(k))
            .collect()
    }
}

fn is_processing(name: &str) -> bool {
    PROCESSING_TYPES.contains(&name)
}

/// Scan a BTLx document (as a string) and summarise it. Never fails on unknown
/// elements — that's the point.
pub fn inspect_str(xml: &str) -> Result<Report, quick_xml::Error> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut version = None;
    let mut language = None;
    let mut parts = 0usize;
    let mut processings: BTreeMap<String, usize> = BTreeMap::new();

    let attr = |e: &quick_xml::events::BytesStart, key: &[u8]| -> Option<String> {
        e.attributes()
            .flatten()
            .find(|a| a.key.as_ref() == key)
            .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
    };

    loop {
        match reader.read_event()? {
            Event::Start(e) | Event::Empty(e) => {
                // Element names carry no prefix (BTLx uses a default namespace).
                let name = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                match name.as_str() {
                    "BTLx" => {
                        version = attr(&e, b"Version");
                        language = attr(&e, b"Language");
                    }
                    "Part" => parts += 1,
                    n if is_processing(n) => *processings.entry(name).or_default() += 1,
                    _ => {}
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(Report {
        version,
        language,
        parts,
        processings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_real_machine_file() {
        // A real BTLx 2.2.0 export vendored in fixtures/samples.
        let xml = include_str!("../fixtures/samples/lignocam-hackathon.btlx");
        let r = inspect_str(xml).unwrap();
        assert_eq!(r.version.as_deref(), Some("2.2.0"));
        assert!(r.parts > 0, "found parts in a real file");
        // This real file mixes JackRafterCut, Lap and Drilling.
        assert!(r.processings.contains_key("JackRafterCut"));
        assert!(r.total_processings() >= 5);
        // We can read Lap/JackRafterCut but not yet serialise them.
        assert!(r.unsupported().contains(&"JackRafterCut"));
    }

    #[test]
    fn inspects_our_own_emitted_file() {
        let xml = include_str!("../fixtures/sample-drilling.btlx");
        let r = inspect_str(xml).unwrap();
        assert_eq!(r.version.as_deref(), Some("2.3.1"));
        assert_eq!(r.parts, 1);
        assert_eq!(r.processings.get("Drilling"), Some(&2));
        assert!(
            r.unsupported().is_empty(),
            "we can serialise everything we emitted"
        );
    }
}
