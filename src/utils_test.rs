use crate::utils::{expand_tilde, zfill};
use std::env;

#[test]
fn test_zfill() {
    assert_eq!(zfill("7", "0", 3), "007");
    assert_eq!(zfill("7", "0", 0), "");
    assert_eq!(zfill("7", "1", 1), "7");
}

#[test]
fn test_expand_tilde() {
    assert_eq!(expand_tilde("~/code"), format!("{}/code", env!("HOME")));
}
