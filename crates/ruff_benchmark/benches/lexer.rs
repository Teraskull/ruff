use std::time::Duration;

use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use ruff_benchmark::{TestCase, TestCaseSpeed, TestFile, TestFileDownloadError};
use ruff_python_parser::lexer::Lexer;
use ruff_python_parser::Mode;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "openbsd"),
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64"
    )
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn create_test_cases() -> Result<Vec<TestCase>, TestFileDownloadError> {
    Ok(vec![
        TestCase::fast(TestFile::try_download("numpy/globals.py", "https://raw.githubusercontent.com/numpy/numpy/89d64415e349ca75a25250f22b874aa16e5c0973/numpy/_globals.py")?),
        TestCase::normal(TestFile::try_download(
            "pydantic/types.py",
            "https://raw.githubusercontent.com/pydantic/pydantic/83b3c49e99ceb4599d9286a3d793cea44ac36d4b/pydantic/types.py",
        )?),
        TestCase::normal(TestFile::try_download("numpy/ctypeslib.py", "https://raw.githubusercontent.com/numpy/numpy/e42c9503a14d66adfd41356ef5640c6975c45218/numpy/ctypeslib.py")?),
        // TestCase::slow(TestFile::try_download(
        //     "large/dataset.py",
        //     "https://raw.githubusercontent.com/DHI/mikeio/b7d26418f4db2909b0aa965253dbe83194d7bb5b/tests/test_dataset.py",
        // )?),
    ])
}

fn benchmark_lexer(criterion: &mut Criterion<WallTime>) {
    let test_cases = create_test_cases().unwrap();
    let mut group = criterion.benchmark_group("lexer");

    for case in test_cases {
        group.throughput(Throughput::Bytes(case.code().len() as u64));
        group.measurement_time(match case.speed() {
            TestCaseSpeed::Fast => Duration::from_secs(10),
            TestCaseSpeed::Normal => Duration::from_secs(20),
            TestCaseSpeed::Slow => Duration::from_secs(45),
        });

        // group.bench_with_input(
        //     BenchmarkId::from_parameter(format!("v0.0.280/{}", case.name())),
        //     &case,
        //     |b, case| {
        //         b.iter(|| {
        //             let parsed = Lexer::new(&case.code(), Mode::Module);
        //             let count = parsed.into_iter().flatten().count();
        //             black_box(count);
        //         });
        //     },
        // );
        //
        // group.bench_with_input(
        //     BenchmarkId::from_parameter(format!("Old/{}", case.name())),
        //     &case,
        //     |b, case| {
        //         b.iter(|| {
        //             let parsed =
        //                 rustpython_parser::lexer::lex(case.code(), rustpython_parser::Mode::Module);
        //             let count = parsed.into_iter().flatten().count();
        //             black_box(count);
        //         });
        //     },
        // );

        group.bench_with_input(
            BenchmarkId::from_parameter(case.name()),
            &case,
            |b, case| {
                b.iter(|| {
                    let parsed = Lexer::new(&case.code(), Mode::Module);
                    let count = parsed.into_iter().flatten().count();
                    black_box(count);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(lexer, benchmark_lexer);
criterion_main!(lexer);
