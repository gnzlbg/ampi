//! MPI test runner.

pub struct Test {
    pub test: fn() -> (),
    pub name: &'static str,
    pub np: usize,
}

/// MPI test runner.
///
/// If no CLI arguments are provided, we are the master runner.
/// Otherwise, the parameter provided is the index of the test to run.
pub fn runner(tests: &[&Test]) {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        master_process(tests);
    } else {
        test_process(tests, args[1].parse().unwrap());
    }
}

/// Spawns each test in a separate process.
fn master_process(tests: &[&Test]) {
    let curr_exe =
        std::env::current_exe().expect("Couldn't get current exe... sorry?");

    let max_name_len = tests.iter().map(|t| t.name.len()).max().unwrap_or(0);

    let mut out = Formatter::new(max_name_len, tests.len());
    out.start();
    for (idx, test) in tests.iter().enumerate() {
        out.running(test.name);
        let output = std::process::Command::new("mpirun")
            .arg("-np")
            .arg(test.np.to_string())
            .arg(&curr_exe)
            .arg(&idx.to_string())
            .output()
            .expect("failed to execute child process");
        if output.status.success() {
            out.success();
        } else {
            out.failure();
            out.error_output.push((
                test.name.to_string(),
                format!(
                    "--- stderr:\n{}\n--- stdout:\n{}",
                    std::str::from_utf8(&output.stdout).unwrap(),
                    std::str::from_utf8(&output.stderr).unwrap()
                ),
            ));
        }
    }
    out.stats();
}

/// Run a specific test
fn test_process(tests: &[&Test], idx: usize) {
    (tests[idx].test)();
    std::process::exit(0);
}

struct Formatter {
    test_count: usize,
    max_len: usize,
    passed: usize,
    failed: usize,
    error_output: Vec<(String, String)>,
}

impl Formatter {
    fn new(max_len: usize, test_count: usize) -> Self {
        Formatter {
            test_count,
            max_len,
            passed: 0,
            failed: 0,
            error_output: Vec::new(),
        }
    }
    fn start(&mut self) {
        use std::io::Write;
        let noun = if self.test_count == 1 {
            "test"
        } else {
            "tests"
        };
        write!(
            std::io::stdout(),
            "\nrunning {count} {noun}\n\n",
            count = self.test_count,
            noun = noun
        )
        .unwrap();
    }
    fn running(&mut self, test_name: &str) {
        use std::io::Write;
        let mut padded_name = test_name.to_string();
        let padding = self.max_len - test_name.len();
        for _ in 0..padding {
            padded_name.push(' ');
        }
        write!(
            std::io::stdout(),
            "test {test_name} ... ",
            test_name = padded_name
        )
        .unwrap();
    }
    fn success(&mut self) {
        use std::io::Write;
        self.passed += 1;
        write!(std::io::stdout(), "ok\n").unwrap();
    }
    fn failure(&mut self) {
        use std::io::Write;
        self.failed += 1;
        write!(std::io::stdout(), "FAILED\n").unwrap();
    }
    fn stats(&mut self) {
        use std::io::Write;
        assert_eq!(self.passed + self.failed, self.test_count);

        for (test_name, output) in &self.error_output {
            write!(
                std::io::stdout(),
                "\nTEST FAILED: {}\n{}\n",
                test_name,
                output
            )
            .unwrap();
        }

        write!(
            std::io::stdout(),
            "tests: {passed} passed; {failed} failed\n\n",
            passed = self.passed,
            failed = self.failed
        )
        .unwrap();
    }
}
