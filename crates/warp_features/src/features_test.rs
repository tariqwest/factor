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

/// Warper approach: Docker sandbox remains a normal feature flag (dogfood),
/// not hard-disabled. PREVIEW/RELEASE still omit it.
#[test]
fn factor_local_docker_sandbox_follows_warper_flag_lists() {
    assert!(
        DOGFOOD_FLAGS.contains(&FeatureFlag::LocalDockerSandbox),
        "DOGFOOD_FLAGS should enable LocalDockerSandbox (Warper parity)"
    );
    for (name, list) in [
        ("PREVIEW_FLAGS", PREVIEW_FLAGS),
        ("RELEASE_FLAGS", RELEASE_FLAGS),
    ] {
        assert!(
            !list.contains(&FeatureFlag::LocalDockerSandbox),
            "{name} must not enable LocalDockerSandbox (Warper parity)"
        );
    }
}

#[cfg(feature = "test-util")]
#[test]
fn factor_local_docker_sandbox_respects_override_and_preference() {
    // Without override, preference can enable (no Factor hard-disable).
    FeatureFlag::LocalDockerSandbox.set_user_preference(true);
    assert!(
        FeatureFlag::LocalDockerSandbox.is_enabled(),
        "user preference should enable LocalDockerSandbox under Warper approach"
    );
    FeatureFlag::LocalDockerSandbox.set_user_preference(false);

    let _override = FeatureFlag::LocalDockerSandbox.override_enabled(true);
    assert!(FeatureFlag::LocalDockerSandbox.is_enabled());
}
