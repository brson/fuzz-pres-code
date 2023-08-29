#![no_main]

use libfuzzer_sys::fuzz_target;
use comrak::{self, Arena, ComrakOptions};

fuzz_target!(|input: &[u8]| {
    if let Ok(input) = std::str::from_utf8(input) {
        let opts = &ComrakOptions::default();
        let arena = &Arena::new();
        let doc = comrak::parse_document(arena, input, opts);
        let mut md = vec![];
        comrak::format_commonmark(doc, opts, &mut md);
    }
});
