#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::disallowed_names)]
#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::regex_creation_in_loops)]
pub(crate) mod cases {
    include!(concat!(env!("OUT_DIR"), "/tests.harness.cases.rs"));

    pub(crate) mod sort {
        include!(concat!(env!("OUT_DIR"), "/tests.harness.cases.sort.rs"));
    }
    pub(crate) mod yet_another_package {
        include!(concat!(
            env!("OUT_DIR"),
            "/tests.harness.cases.yet_another_package.rs"
        ));
    }
    pub(crate) mod other_package {
        include!(concat!(
            env!("OUT_DIR"),
            "/tests.harness.cases.other_package.rs"
        ));
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub(crate) mod harness {
    include!(concat!(env!("OUT_DIR"), "/tests.harness.rs"));
}
