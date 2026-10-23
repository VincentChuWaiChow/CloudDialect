use clouddialect_contracts::{
    CatalogSnapshot, ManifestOutcome, ManifestValidator, ValidateManifest,
};

#[test]
fn trait_dispatch_uses_the_real_local_validator_without_authority_projection() {
    let validator = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let local: &dyn ValidateManifest = &validator;
    let result = local.validate_manifest(b"");
    assert_eq!(result, validator.validate_manifest(b""));
    assert_eq!(result.outcome, ManifestOutcome::Fail);
    assert_eq!(result.structural_errors.len(), 1);
    assert!(result.semantic_errors.is_empty());
}
