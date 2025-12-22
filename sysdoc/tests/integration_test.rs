use std::path::PathBuf;

fn get_workspace_root() -> PathBuf {
    // Get the workspace root by going up from the manifest directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).parent().unwrap().to_path_buf()
}

#[test]
fn test_minimal_sdd_exists() {
    let workspace_root = get_workspace_root();
    let example_path = workspace_root.join("examples");
    assert!(example_path.exists(), "examples directory should exist");
    assert!(
        example_path
            .join("minimal-sdd/src/01-introduction/01.01_purpose.md")
            .exists(),
        "minimal-sdd should have introduction section"
    );
    assert!(
        example_path
            .join("minimal-sdd/src/02-architecture/02.01_overview.md")
            .exists(),
        "minimal-sdd should have architecture section"
    );
    assert!(
        example_path
            .join("minimal-sdd/src/02-architecture/system-diagram.drawio.svg")
            .exists(),
        "minimal-sdd should have diagram"
    );
}

#[test]
fn test_complete_sdd_exists() {
    let workspace_root = get_workspace_root();
    let example_path = workspace_root.join("examples");
    assert!(example_path.exists(), "examples directory should exist");
    assert!(
        example_path
            .join("complete-sdd/src/02-architecture/tables/components.csv")
            .exists(),
        "complete-sdd should have CSV tables"
    );
    assert!(
        example_path
            .join("complete-sdd/src/02-architecture/diagrams/system-context.drawio.svg")
            .exists(),
        "complete-sdd should have diagrams"
    );
    assert!(
        example_path
            .join("complete-sdd/src/03-detailed-design/ui-screenshot.png")
            .exists(),
        "complete-sdd should have PNG images"
    );
}

#[test]
fn test_template_exists() {
    let workspace_root = get_workspace_root();
    let template_path = workspace_root.join("examples");
    assert!(template_path.exists(), "examples directory should exist");
    assert!(
        template_path
            .join("templates/src/DI-IPSC-81435B/01-scope/01.01_identification.md")
            .exists(),
        "template should have scope section"
    );
    assert!(
        template_path
            .join("templates/src/DI-IPSC-81435B/03-software-design/03.01_system-wide-design.md")
            .exists(),
        "template should have software design sections"
    );
}

/// Test that all fixture directories have the expected structure
#[test]
fn test_fixtures_exist() {
    let workspace_root = get_workspace_root();
    let fixtures_path = workspace_root.join("tests/fixtures");

    let test_cases = [
        "test-normal-text",
        "test-italics",
        "test-bold",
        "test-strikethrough",
        "test-png-image",
        "test-svg-image",
        "test-csv-table",
        "test-inline-table",
    ];

    for test_case in test_cases {
        let test_dir = fixtures_path.join(test_case);
        assert!(
            test_dir.join("sysdoc.toml").exists(),
            "{} should have sysdoc.toml",
            test_case
        );
        assert!(
            test_dir.join("src/01_test.md").exists(),
            "{} should have src/01_test.md",
            test_case
        );
    }

    // Check specific assets for image and table tests
    assert!(
        fixtures_path
            .join("test-png-image/src/test-image.png")
            .exists(),
        "PNG test should have test-image.png"
    );
    assert!(
        fixtures_path
            .join("test-svg-image/src/test-image.drawio.svg")
            .exists(),
        "SVG test should have test-image.drawio.svg"
    );
    assert!(
        fixtures_path
            .join("test-csv-table/src/test-data.csv")
            .exists(),
        "CSV test should have test-data.csv"
    );
}
