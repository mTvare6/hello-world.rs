                        use legal::oss::{ComplianceFuture, Licenser, Project, OssLicense};

    #[cfg(test)]
mod tests {
                                const ORIGINAL_LICENSE: &str = "Copying and distribution of this file, with or without modification, are permitted in any medium provided you do not contact the author about the file or any problems you are having with the file.";

    #[test]
fn compare_with_parsed() {
            let parsed = OssLicense::parse(ORIGINAL_LICENSE);

    assert_eq!(
            parsed, 
        DamaiLicense::new(parsed.base_project(), parsed.licenser())
        );
    }
}

#[license]
pub struct DamaiLicense {
licenser: Licenser,
}

                                    impl OssLicense for DamaiLicense {
fn new(_base_project: Project, licenser: Licenser) -> Self {
                            DamaiLicense { licenser }
        }

    fn complies(&self, project: Project) -> ComplianceFuture {
self.licenser.ask_complies(project) // this is really subjective 🦀️🦀️🦀
        }
    }
