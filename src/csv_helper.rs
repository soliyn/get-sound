use std::io::Read;
use csv;
use csv::ReaderBuilder;

pub fn get_words<T: Read>(r: T, column_index: usize) -> Vec<String> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_reader(r);

    reader
        .records()
        .filter_map(|r|
            r.ok()?
                .get(column_index)
                .map(|s| s.to_string())
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_words() {
        let data = "\
#separator:tab
#html:false
throe	θrəʊ
spruce	spruːs
faux	fəʊ
";
        let words = get_words(data.as_bytes(), 0);
        assert_eq!(words, vec!["throe", "spruce", "faux"]);
    }
}