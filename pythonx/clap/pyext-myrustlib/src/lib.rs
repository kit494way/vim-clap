#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};
use rff::match_and_score_with_positions;
use std::mem;

fn fuzzy_match(
    _py: Python,
    query: &str,
    candidates: Vec<String>,
) -> PyResult<(Vec<Vec<usize>>, Vec<String>)> {
    let scorer = |line: &str| {
        match_and_score_with_positions(&query, line).map(|(_, score, indices)| (score, indices))
    };

    let mut ranked = candidates
        .into_iter()
        .filter_map(|line| scorer(&line).map(|(score, indices)| (line, score, indices)))
        .collect::<Vec<_>>();

    ranked.sort_unstable_by(|(_, v1, _), (_, v2, _)| v2.partial_cmp(v1).unwrap());

    let mut indices = Vec::new();
    let mut filtered = Vec::new();
    for (text, _, ids) in ranked.iter() {
        indices.push(ids.clone());
        filtered.push(text.clone());
    }

    Ok((indices, filtered))
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m| {
    r#try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    r#try!(m.add(
        py,
        "fuzzy_match",
        py_fn!(py, fuzzy_match(query: &str, candidates: Vec<String>)),
    ));
    Ok(())
});
