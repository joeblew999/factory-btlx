//! Emit a sample BTLx document to stdout — one part exercising every processing
//! this crate can serialise.
//!
//! ```sh
//! cargo run --example emit > fixtures/sample.btlx
//! xmllint --noout --schema fixtures/schema/BTLx_2_3_1.offline.xsd fixtures/sample.btlx
//! ```

use factory_btlx::btlx::{model::*, to_xml};

fn main() -> anyhow::Result<()> {
    // A 3 m beam, 160×80 mm, carrying a rafter cut, a lap, a mortise-and-tenon and a
    // bolt hole — the processings that dominate real machine files.
    let part = Part::new(3000.0, 160.0, 80.0)
        .designation("beam-1")
        .with_processings(vec![
            Processing::JackRafterCut(JackRafterCut::new(
                "rafter-cut",
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
                "mortise",
                3,
                RefPlane::Global(1),
                1500.0,
                40.0,
                100.0,
                40.0,
                60.0,
            )),
            Processing::Tenon(Tenon::new(
                "tenon",
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

    let doc = Btlx::new(Project::new("sample-project", vec![part]));
    print!("{}", to_xml(&doc)?);
    Ok(())
}
