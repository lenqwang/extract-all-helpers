#![deny(clippy::all)]

use std::{collections::HashSet, sync::{mpsc::channel}};

use get_helpers::get_helpers;
use threadpool::ThreadPool;

mod utils;
mod get_helpers;

#[macro_use]
extern crate napi_derive;

extern crate num_cpus;
extern crate threadpool;

#[napi]
pub fn get_all_helpers(file_path: String) -> Vec<String> {
  let helpers: Vec<_> = get_helpers(file_path.as_str()).into_iter().collect();
  
  helpers
}

#[napi]
pub fn extract_all_helpers(files: Vec<String>) -> Vec<String> {
  let mut hash_set = HashSet::new();
  let num_cors = num_cpus::get();
  let number_threads = num_cors;
  let file_count = files.len();
  let files_per_thread = if number_threads > file_count {
    file_count
  } else {
    ((file_count / number_threads) as f32).ceil() as usize
  };
  let pool = ThreadPool::new(number_threads);

  let (tx, rx) = channel();
  let mut chunks = Vec::new();

  for chunk in files.chunks(files_per_thread) {
    chunks.push(chunk.to_owned());
  }
  
  for chunk in chunks {
    let tx = tx.clone();

    pool.execute(move || {
      for file_path in chunk {
        let helpers = get_helpers(&file_path);
        tx.send(helpers).expect("Cannot send helper to channel, Please check the channel is abort or not");
      }
    })
  }

  drop(tx);

  rx.iter().for_each(|helpers| {
    hash_set.extend(helpers);
  });

  hash_set.into_iter().collect()
}