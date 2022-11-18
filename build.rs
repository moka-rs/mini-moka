#[cfg(skeptic)]
fn main() {
    // generates doc tests for `README.md`.
    skeptic::generate_doc_tests(&["README.md"]);
}

#[cfg(not(any(skeptic)))]
fn main() {}
