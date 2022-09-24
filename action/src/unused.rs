// Ported from arduino/setup-protoc.
// Hopefully we can just use semver crate instead of this.
fn _normalize_version(version: String) -> String {
    let pre_strings = ["beta", "rc", "preview"];
    let mut version_part: Vec<_> = version.split('.').collect();
    match version_part.get(1) {
        Some(part) => {
            let has_pre_string = pre_strings.iter().any(|ps| part.contains(ps));
            if has_pre_string {
                // handle beta and rc
                // e.g. 1.10beta1 -? 1.10.0-beta1, 1.10rc1 -> 1.10.0-rc1
                let temp = &part
                    .replace("beta", ".0-beta")
                    .replace("rc", ".0-rc")
                    .replace("preview", ".0-preview");
                version_part[1] = temp;
                return version_part.join(".");
            }
        }
        None => {
            // append minor and patch version if not available
            // e.g. 2 -> 2.0.0
            return format!("{version}.0.0");
        }
    }

    match version_part.get(2) {
        None => {
            //append patch version if not available
            // e.g. 2.1 -> 2.1.0
            return format!("{version}.0");
        }
        Some(part) => {
            if pre_strings.iter().any(|ps| part.contains(ps)) {
                // handle beta and rc
                // e.g. 1.8.5beta1 -> 1.8.5-beta1, 1.8.5rc1 -> 1.8.5-rc1
                let temp = &part
                    .replace("beta", "-beta")
                    .replace("rc", "-rc")
                    .replace("preview", "-preview");
                version_part[2] = temp;
                return version_part.join(".");
            }
        }
    }

    version
}
