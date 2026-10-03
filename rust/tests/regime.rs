//! Every opt-in feature must have its own suite file. A base-world test run is not enough.

use std::path::Path;

use suvorov_core::Feature;

#[test]
fn every_feature_has_an_isolated_suite_file() {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    for feature in Feature::ALL {
        let path = tests.join(feature.suite_file());
        assert!(
            path.is_file(),
            "missing isolated suite for {feature:?}: {}",
            path.display()
        );
    }
}
