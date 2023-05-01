use notify::*;
use std::env;
use std::fs;
use std::{path::Path, time::Duration};

use crate::parse_to_lang::parse;
mod parse_to_lang;

// example of detecting the recommended watcher kind
fn main() {
    println!("start loop:");
    let (tx, rx) = std::sync::mpsc::channel();
    // This example is a little bit misleading as you can just create one Config and use it for all watchers.
    // That way the pollwatcher specific stuff is still configured, if it should be used.
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        // custom config for PollWatcher kind
        // you
        let config = Config::default().with_poll_interval(Duration::from_secs(1));
        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        // use default config for everything else
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    // watch some stuff
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    for e in rx {
        if let Ok(Event { kind: _, paths, attrs: _ }) = e {
            //println!("path is ${:?}",paths[0].as_path());
            //if !paths[0].as_path().ends_with(&".txt") {continue;}
            let contents =
                fs::read_to_string(paths[0].as_path()).expect("Should have been able to read the file");
            parse(contents);
        }
    }
}
