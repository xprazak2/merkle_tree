use itertools::{Itertools, Either};

use crate::utils::create_hash;

pub fn naive_merkle_root(input: Vec<&str>) -> String {
    let preprocessed = preprocess(input);

    let res = process(preprocessed);
    return res
}

fn process(input: Vec<String>) -> String {
  if input.len() == 1 {
    return input[0].to_owned();
  }

  let mut res = vec![];

  let (first, second): (Vec<_>, Vec<_>) = input.into_iter().enumerate().partition_map(|(idx, item)| {
    if idx % 2 == 0 {
      Either::Left(item)
    } else {
      Either::Right(item)
    }
  });

  // zip with hash, handle odd length

  process(res)
}

fn preprocess(input: Vec<&str>) -> Vec<String> {
  // maybe we could return just Vec<Vec<u8>>
  input.into_iter().map(|item| create_hash(item)).collect()
}