use crate::package::Packages;


pub fn check_dependency(dependency: String, min_ver: String) -> bool {
    // first see if the dependency is already installed
    let mut packages = Packages::new();
    packages.load().unwrap();

    if packages.has_package(dependency.clone()) {
        // check to see if the version is greater than or equal to the minimum version
        let package = packages.get_package(dependency).unwrap();
        let version = package.version.clone();

        // check to see if the version is greater than or equal to the minimum version
        if crate::package::compare_versions(version, min_ver) < 0 {
            return false;
        }
    } else {
        return false
    }

    true
}