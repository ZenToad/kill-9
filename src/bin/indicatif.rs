#![allow(dead_code)]
#![allow(unused_imports)]
use console::{Emoji, Style, Term, style};
use indicatif::{
    HumanDuration, MultiProgress, ProgressBar, ProgressIterator, ProgressState, ProgressStyle,
};
use rand::Rng;
use rand::prelude::IndexedRandom;
use std::{
    cmp::min,
    fmt::Write,
    io::{BufRead, BufReader},
    process,
    process::{Command, Stdio},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};
use tokio::runtime;
use tokio::time::interval;

use once_cell::sync::Lazy;
use rand::RngCore;
use rand::rngs::ThreadRng;
use std::fmt::Debug;
fn main() {
    // iterator();
    // cargo();
    // cargo_wrap();
    // download();
    // fastbar();
    // finebars();
    // log();
    long_spinner();
    // morebars();
    // multi();
    // multi_tree();
    // single();
    // tokio();
    // yarnish();
}

#[derive(Debug, Clone)]
enum Action {
    AddProgressBar(usize),
    IncProgressBar(usize),
}

#[derive(Clone, Debug)]
struct Elem {
    key: String,
    index: usize,
    indent: usize,
    progress_bar: ProgressBar,
}

static ELEMENTS: Lazy<[Elem; 9]> = Lazy::new(|| {
    [
        Elem {
            indent: 1,
            index: 0,
            progress_bar: ProgressBar::new(32),
            key: "jumps".to_string(),
        },
        Elem {
            indent: 2,
            index: 1,
            progress_bar: ProgressBar::new(32),
            key: "lazy".to_string(),
        },
        Elem {
            indent: 0,
            index: 0,
            progress_bar: ProgressBar::new(32),
            key: "the".to_string(),
        },
        Elem {
            indent: 3,
            index: 3,
            progress_bar: ProgressBar::new(32),
            key: "dog".to_string(),
        },
        Elem {
            indent: 2,
            index: 2,
            progress_bar: ProgressBar::new(32),
            key: "over".to_string(),
        },
        Elem {
            indent: 2,
            index: 1,
            progress_bar: ProgressBar::new(32),
            key: "brown".to_string(),
        },
        Elem {
            indent: 1,
            index: 1,
            progress_bar: ProgressBar::new(32),
            key: "quick".to_string(),
        },
        Elem {
            indent: 3,
            index: 5,
            progress_bar: ProgressBar::new(32),
            key: "a".to_string(),
        },
        Elem {
            indent: 3,
            index: 3,
            progress_bar: ProgressBar::new(32),
            key: "fox".to_string(),
        },
    ]
});
/// The function guarantees to return the action, that is valid for the current tree.
fn get_action(rng: &mut dyn RngCore, tree: &Mutex<Vec<&Elem>>) -> Option<Action> {
    let elem_len = ELEMENTS.len() as u64;
    let list_len = tree.lock().unwrap().len() as u64;
    let sum_free = tree
        .lock()
        .unwrap()
        .iter()
        .map(|e| {
            let pos = e.progress_bar.position();
            let len = e.progress_bar.length().unwrap();
            len - pos
        })
        .sum::<u64>();

    if sum_free == 0 && list_len == elem_len {
        // nothing to do more
        None
    } else if sum_free == 0 && list_len < elem_len {
        // there is no place to make an increment
        Some(Action::AddProgressBar(tree.lock().unwrap().len()))
    } else {
        loop {
            let list = tree.lock().unwrap();
            let k = rng.random_range(0..17);
            if k == 0 && list_len < elem_len {
                return Some(Action::AddProgressBar(list.len()));
            } else {
                let l = (k % list_len) as usize;
                let pos = list[l].progress_bar.position();
                let len = list[l].progress_bar.length();
                if pos < len.unwrap() {
                    return Some(Action::IncProgressBar(l));
                }
            }
        }
    }
}

fn multi_tree() {
    let mp = Arc::new(MultiProgress::new());
    let sty_main = ProgressStyle::with_template("{bar:40.green/yellow} {pos:>4}/{len:4}").unwrap();
    let sty_aux = ProgressStyle::with_template("{spinner:.green} {msg} {pos:>4}/{len:4}").unwrap();

    let pb_main = mp.add(ProgressBar::new(
        ELEMENTS
            .iter()
            .map(|e| e.progress_bar.length().unwrap())
            .sum(),
    ));
    pb_main.set_style(sty_main);
    for elem in ELEMENTS.iter() {
        elem.progress_bar.set_style(sty_aux.clone());
    }

    let tree: Arc<Mutex<Vec<&Elem>>> = Arc::new(Mutex::new(Vec::with_capacity(ELEMENTS.len())));
    let tree2 = Arc::clone(&tree);

    let mp2 = Arc::clone(&mp);
    let _ = thread::spawn(move || {
        let mut rng = ThreadRng::default();
        pb_main.tick();
        loop {
            thread::sleep(Duration::from_millis(15));
            match get_action(&mut rng, &tree) {
                None => {
                    // all elements were exhausted
                    pb_main.finish();
                    return;
                }
                Some(Action::AddProgressBar(el_idx)) => {
                    let elem = &ELEMENTS[el_idx];
                    let pb = mp2.insert(elem.index + 1, elem.progress_bar.clone());
                    pb.set_message(format!("{}  {}", "  ".repeat(elem.indent), elem.key));
                    tree.lock().unwrap().insert(elem.index, elem);
                }
                Some(Action::IncProgressBar(el_idx)) => {
                    let elem = &tree.lock().unwrap()[el_idx];
                    elem.progress_bar.inc(1);
                    let pos = elem.progress_bar.position();
                    if pos >= elem.progress_bar.length().unwrap() {
                        elem.progress_bar.finish_with_message(format!(
                            "{}{} {}",
                            "  ".repeat(elem.indent),
                            "✔",
                            elem.key
                        ));
                    }
                    pb_main.inc(1);
                }
            }
        }
    })
    .join();

    println!("===============================");
    println!("the tree should be the same as:");
    for elem in tree2.lock().unwrap().iter() {
        println!("{}  {}", "  ".repeat(elem.indent), elem.key);
    }
}

fn multi() {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let n = 200;
    let pb = m.add(ProgressBar::new(n));
    pb.set_style(sty.clone());
    pb.set_message("todo");
    let pb2 = m.add(ProgressBar::new(n));
    pb2.set_style(sty.clone());
    pb2.set_message("finished");

    let pb3 = m.insert_after(&pb2, ProgressBar::new(1024));
    pb3.set_style(sty);

    m.println("starting!").unwrap();

    let mut threads = vec![];

    let m_clone = m.clone();
    let h3 = thread::spawn(move || {
        for i in 0..1024 {
            thread::sleep(Duration::from_millis(2));
            pb3.set_message(format!("item #{}", i + 1));
            pb3.inc(1);
        }
        m_clone.println("pb3 is done!").unwrap();
        pb3.finish_with_message("done");
    });

    for i in 0..n {
        thread::sleep(Duration::from_millis(15));
        if i == n / 3 {
            thread::sleep(Duration::from_secs(2));
        }
        pb.inc(1);
        let pb2 = pb2.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(rand::rng().random_range(Duration::from_secs(1)..Duration::from_secs(5)));
            pb2.inc(1);
        }));
    }
    pb.finish_with_message("all jobs started");

    for thread in threads {
        let _ = thread.join();
    }
    let _ = h3.join();
    pb2.finish_with_message("all jobs done");
    m.clear().unwrap();
}

static PACKAGES: &[&str] = &[
    "fs-events",
    "my-awesome-module",
    "emoji-speaker",
    "wrap-ansi",
    "stream-browserify",
    "acorn-dynamic-import",
];

static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "make clean",
    "gcc foo.c -o foo",
    "gcc bar.c -o bar",
    "./helper.sh rebuild-cache",
    "make all-clean",
    "make test",
];

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn yarnish() {
    let mut rng = rand::rng();
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!(
        "{} {}Resolving packages...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    println!(
        "{} {}Fetching packages...",
        style("[2/4]").bold().dim(),
        TRUCK
    );

    println!(
        "{} {}Linking dependencies...",
        style("[3/4]").bold().dim(),
        CLIP
    );
    let deps = 1232;
    let pb = ProgressBar::new(deps);
    for _ in 0..deps {
        thread::sleep(Duration::from_millis(3));
        pb.inc(1);
    }
    pb.finish_and_clear();

    println!(
        "{} {}Building fresh packages...",
        style("[4/4]").bold().dim(),
        PAPER
    );
    let m = MultiProgress::new();
    let handles: Vec<_> = (0..4u32)
        .map(|i| {
            let count = rng.random_range(30..80);
            let pb = m.add(ProgressBar::new(count));
            pb.set_style(spinner_style.clone());
            pb.set_prefix(format!("[{}/?]", i + 1));
            thread::spawn(move || {
                let mut rng = rand::rng();
                let pkg = PACKAGES.choose(&mut rng).unwrap();
                for _ in 0..count {
                    let cmd = COMMANDS.choose(&mut rng).unwrap();
                    thread::sleep(Duration::from_millis(rng.random_range(25..200)));
                    pb.set_message(format!("{pkg}: {cmd}"));
                    pb.inc(1);
                }
                pb.finish_with_message("waiting...");
            })
        })
        .collect();
    for h in handles {
        let _ = h.join();
    }
    m.clear().unwrap();

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}

fn tokio() {
    // Plain progress bar, totaling 1024 steps.
    let steps = 1024;
    let pb = ProgressBar::new(steps);

    // Stream of events, triggering every 5ms.
    let rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("failed to create runtime");

    // Future computation which runs for `steps` interval events,
    // incrementing one step of the progress bar each time.
    let future = async {
        let mut intv = interval(Duration::from_millis(5));

        for _ in 0..steps {
            intv.tick().await;
            pb.inc(1);
        }
    };

    // Drive the future to completion, blocking until done.
    rt.block_on(future);

    // Mark the progress bar as finished.
    pb.finish();
}

fn single() {
    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        thread::sleep(Duration::from_millis(5));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn morebars() {
    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template("{bar:40.green/yellow} {pos:>7}/{len:7}").unwrap();

    let pb = m.add(ProgressBar::new(5));
    pb.set_style(sty.clone());

    // make sure we show up at all.  otherwise no rendering
    // event.
    pb.tick();
    for _ in 0..5 {
        let pb2 = m.add(ProgressBar::new(128));
        pb2.set_style(sty.clone());
        for _ in 0..128 {
            thread::sleep(Duration::from_millis(5));
            pb2.inc(1);
        }
        pb2.finish();
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn long_spinner() {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message("Calculating...");
    thread::sleep(Duration::from_secs(5));
    pb.finish_with_message("Done");
}

fn log() {
    let pb = ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(Duration::from_millis(25));
        pb.println(format!("[+] finished #{i}"));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn finebars() {
    let styles = [
        ("Rough bar:", "█  ", "red"),
        ("Fine bar: ", "█▉▊▋▌▍▎▏  ", "yellow"),
        ("Vertical: ", "█▇▆▅▄▃▂▁  ", "green"),
        ("Fade in:  ", "█▓▒░  ", "blue"),
        ("Blocky:   ", "█▛▌▖  ", "magenta"),
    ];

    let m = MultiProgress::new();

    let handles: Vec<_> = styles
        .iter()
        .map(|s| {
            let pb = m.add(ProgressBar::new(512));
            pb.set_style(
                ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", s.2))
                    .unwrap()
                    .progress_chars(s.1),
            );
            pb.set_prefix(s.0);
            let wait = Duration::from_millis(rand::rng().random_range(10..30));
            thread::spawn(move || {
                for i in 0..512 {
                    thread::sleep(wait);
                    pb.inc(1);
                    pb.set_message(format!("{:3}%", 100 * i / 512));
                }
                pb.finish_with_message("100%");
            })
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }
}

fn many_units_of_easy_work(n: u64, label: &str) {
    let pb = ProgressBar::new(n);

    let mut sum = 0;
    for i in 0..n {
        // Any quick computation, followed by an update to the progress bar.
        sum += 2 * i + 3;
        pb.inc(1);
    }
    pb.finish();

    println!("[{}] Sum ({}) calculated in {:?}", label, sum, pb.elapsed());
}

fn fastbar() {
    const N: u64 = 1 << 20;

    // Perform a long sequence of many simple computations monitored by a
    // default progress bar.
    many_units_of_easy_work(N, "Default progress bar ");
}

fn download() {
    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}

fn cargo_wrap() {
    let started = Instant::now();

    println!("Compiling package in release mode...");

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} cargo: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );

    let mut p = process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .stderr(process::Stdio::piped())
        .spawn()
        .unwrap();

    for line in BufReader::new(p.stderr.take().unwrap()).lines() {
        let line = line.unwrap();
        let stripped_line = line.trim();
        if !stripped_line.is_empty() {
            pb.set_message(stripped_line.to_owned());
        }
        pb.tick();
    }

    p.wait().unwrap();

    pb.finish_and_clear();

    println!("Done in {}", HumanDuration(started.elapsed()));
}

static CRATES: &[(&str, &str)] = &[
    ("console", "v0.14.1"),
    ("lazy_static", "v1.4.0"),
    ("libc", "v0.2.93"),
    ("regex", "v1.4.6"),
    ("regex-syntax", "v0.6.23"),
    ("terminal_size", "v0.1.16"),
    ("libc", "v0.2.93"),
    ("unicode-width", "v0.1.8"),
    ("lazy_static", "v1.4.0"),
    ("number_prefix", "v0.4.0"),
    ("regex", "v1.4.6"),
    ("rand", "v0.8.3"),
    ("getrandom", "v0.2.2"),
    ("cfg-if", "v1.0.0"),
    ("libc", "v0.2.93"),
    ("rand_chacha", "v0.3.0"),
    ("ppv-lite86", "v0.2.10"),
    ("rand_core", "v0.6.2"),
    ("getrandom", "v0.2.2"),
    ("rand_core", "v0.6.2"),
    ("tokio", "v1.5.0"),
    ("bytes", "v1.0.1"),
    ("pin-project-lite", "v0.2.6"),
    ("slab", "v0.4.3"),
    ("indicatif", "v0.15.0"),
];

fn cargo() {
    const NUM_CPUS: usize = 4;
    let start = Instant::now();

    // mimic cargo progress bar although it behaves a bit different
    let pb = ProgressBar::new(CRATES.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            // note that bar size is fixed unlike cargo which is dynamic
            // and also the truncation in cargo uses trailers (`...`)
            if Term::stdout().size().1 > 80 {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}"
            } else {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
            },
        )
        .unwrap()
        .progress_chars("=> "),
    );
    pb.set_prefix("Building");

    // process in another thread
    // crates to be iterated but not exactly a tree
    let crates = Arc::new(Mutex::new(CRATES.iter()));
    let (tx, rx) = mpsc::channel();
    for n in 0..NUM_CPUS {
        let tx = tx.clone();
        let crates = crates.clone();
        thread::spawn(move || {
            let mut rng = rand::rng();
            loop {
                let krate = crates.lock().unwrap().next();
                // notify main thread if n thread is processing a crate
                tx.send((n, krate)).unwrap();
                if let Some(krate) = krate {
                    thread::sleep(Duration::from_millis(
                        // last compile and linking is always slow, let's mimic that
                        if CRATES.last() == Some(krate) {
                            rng.random_range(1_000..2_000)
                        } else {
                            rng.random_range(250..1_000)
                        },
                    ));
                } else {
                    break;
                }
            }
        });
    }
    // drop tx to stop waiting
    drop(tx);

    let green_bold = Style::new().green().bold();

    // do progress drawing in main thread
    let mut processing = [None; NUM_CPUS];
    while let Ok((n, krate)) = rx.recv() {
        processing[n] = krate;
        let crates: Vec<&str> = processing
            .iter()
            .filter_map(|t| t.copied().map(|(name, _)| name))
            .collect();
        pb.set_message(crates.join(", "));
        if let Some((name, version)) = krate {
            // crate is being built
            let line = format!(
                "{:>12} {} {}",
                green_bold.apply_to("Compiling"),
                name,
                version
            );
            pb.println(line);

            pb.inc(1);
        }
    }
    pb.finish_and_clear();

    // compilation is finished
    println!(
        "{:>12} dev [unoptimized + debuginfo] target(s) in {}",
        green_bold.apply_to("Finished"),
        HumanDuration(start.elapsed())
    );
}

fn iterator() {
    // Default styling, attempt to use Iterator::size_hint to count input size
    for _ in (0..1000).progress() {
        // ...
        thread::sleep(Duration::from_millis(5));
    }

    // Provide explicit number of elements in iterator
    for _ in (0..1000).progress_count(1000) {
        // ...
        thread::sleep(Duration::from_millis(5));
    }

    // Provide a custom bar style
    let pb = ProgressBar::new(1000);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
    for _ in (0..1000).progress_with(pb) {
        // ...
        thread::sleep(Duration::from_millis(5));
    }
}
