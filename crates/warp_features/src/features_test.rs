use super::*;

#[test]
#[ignore = "CORE-3768 - need to clean up PREVIEW_FLAGS, but this is a temporary fix for the cluttered changelog"]
fn test_all_preview_flags_have_a_description() {
    for flag in PREVIEW_FLAGS {
        assert!(
            flag.flag_description()
                .is_some_and(|description| !description.is_empty()),
            "Missing description for preview-enabled flag {flag:?}"
        );
    }
}

/// Factor P-01: Docker sandbox must stay off unless a test override forces it.
#[test]
fn factor_local_docker_sandbox_is_hard_disabled_without_override() {
    FeatureFlag::LocalDockerSandbox.set_user_preference(true);
    assert!(
        !FeatureFlag::LocalDockerSandbox.is_enabled(),
        "user preference must not re-enable dropped Docker sandbox product"
    );
    // Clear preference so other tests are not affected.
    FeatureFlag::LocalDockerSandbox.set_user_preference(false);
}

#[cfg(feature = "test-util")]
#[test]
fn factor_local_docker_sandbox_test_override_still_works() {
    let _override = FeatureFlag::LocalDockerSandbox.override_enabled(true);
    assert!(
        FeatureFlag::LocalDockerSandbox.is_enabled(),
        "test override must still work while residual code is under test"
    );
}

#[test]
fn factor_local_docker_sandbox_absent_from_shipped_flag_lists() {
    for (name, list) in [
        ("DOGFOOD_FLAGS", DOGFOOD_FLAGS),
        ("PREVIEW_FLAGS", PREVIEW_FLAGS),
        ("RELEASE_FLAGS", RELEASE_FLAGS),
        ("DEBUG_FLAGS", DEBUG_FLAGS),
    ] {
        assert!(
            !list.contains(&FeatureFlag::LocalDockerSandbox),
            "{name} must not enable LocalDockerSandbox"
        );
    }
}
