if __name__ == "__main__":
    import importlib.metadata
    import shutil

    import trove_classifiers

    trove_version = importlib.metadata.distribution("trove_classifiers").version

    with open("lib.rs") as lib_rs, open(".lib.rs", "w") as _lib_rs:
        for line in lib_rs:
            if line.startswith("pub const PYPA_VERSION"):
                line = f'pub const PYPA_VERSION: &str = "{trove_version}";\n'
            if line == "pub enum Classifier {\n":
                _lib_rs.write(line)
                while (line := next(lib_rs)) != "}\n": pass
                for classifier in trove_classifiers.sorted_classifiers:
                    member_name = (classifier
                        .replace("::", "__")
                        .replace(".", "_")
                        .replace(" ", "")
                        .replace("(", "")
                        .replace(")", "")
                        .replace("/", "")
                        .replace("-", "")
                        .replace("'", "")
                        .replace("#", "sharp")
                        .replace("+", "plus"))
                    _lib_rs.write(f'    #[strum(serialize = "{classifier}")]\n')
                    _lib_rs.write(f"    {member_name},\n")
            _lib_rs.write(line)
    shutil.copy(".lib.rs", "lib.rs")
