use std::env;
use std::process::{Command, Output};
use std::str;

// List of cfgs this build script is allowed to set. The list is needed to support check-cfg, as we
// need to know all the possible cfgs that this script will set. If you need to set another cfg
// make sure to add it to this list as well.
const ALLOWED_CFGS: &'static [&'static str] = &[
    "emscripten_new_stat_abi",
    "espidf_time32",
    "freebsd10",
    "freebsd11",
    "freebsd12",
    "freebsd13",
    "freebsd14",
    "freebsd15",
    "libc_align",
    "libc_cfg_target_vendor",
    "libc_const_extern_fn",
    "libc_const_extern_fn_unstable",
    "libc_const_size_of",
    "libc_core_cvoid",
    "libc_deny_warnings",
    "libc_int128",
    "libc_long_array",
    "libc_non_exhaustive",
    "libc_packedN",
    "libc_priv_mod_use",
    "libc_ptr_addr_of",
    "libc_thread_local",
    "libc_underscore_const_names",
    "libc_union",
    "libc_ctest",
];

// Extra values to allow for check-cfg.
const CHECK_CFG_EXTRA: &'static [(&'static str, &'static [&'static str])] = &[
    (
        "target_os",
        &[
            "switch", "aix", "ohos", "hurd", "rtems", "visionos", "nuttx", "blueos",
        ],
    ),
    ("target_env", &["illumos", "wasi", "aix", "ohos"]),
    (
        "target_arch",
        &["loongarch64", "mips32r6", "mips64r6", "csky"],
    ),
];

fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");

    let (rustc_minor_ver, is_nightly) = rustc_minor_nightly();
    let rustc_dep_of_std = env::var("CARGO_FEATURE_RUSTC_DEP_OF_STD").is_ok();
    let align_cargo_feature = env::var("CARGO_FEATURE_ALIGN").is_ok();
    let const_extern_fn_cargo_feature = env::var("CARGO_FEATURE_CONST_EXTERN_FN").is_ok();
    let libc_ci = env::var("LIBC_CI").is_ok();
    let libc_check_cfg = env::var("LIBC_CHECK_CFG").is_ok() || rustc_minor_ver >= 80;

    if env::var("CARGO_FEATURE_USE_STD").is_ok() {
        println!(
            "cargo:warning=\"libc's use_std cargo feature is deprecated since libc 0.2.55; \
             please consider using the `std` cargo feature instead\""
        );
    }

    // The ABI of libc used by std is backward compatible with FreeBSD 12.
    // The ABI of libc from crates.io is backward compatible with FreeBSD 11.
    //
    // On CI, we detect the actual FreeBSD version and match its ABI exactly,
    // running tests to ensure that the ABI is correct.
    let which_freebsd = if libc_ci {
        which_freebsd().unwrap_or(11)
    } else if rustc_dep_of_std {
        12
    } else {
        11
    };
    match which_freebsd {
        x if x < 10 => panic!("FreeBSD older than 10 is not supported"),
        10 => set_cfg("freebsd10"),
        11 => set_cfg("freebsd11"),
        12 => set_cfg("freebsd12"),
        13 => set_cfg("freebsd13"),
        14 => set_cfg("freebsd14"),
        _ => set_cfg("freebsd15"),
    }

    match emcc_version_code() {
        Some(v) if (v >= 30142) => set_cfg("emscripten_new_stat_abi"),
        // Non-Emscripten or version < 3.1.42.
        Some(_) | None => (),
    }

    // On CI: deny all warnings
    if libc_ci {
        set_cfg("libc_deny_warnings");
    }

    // Rust >= 1.15 supports private module use:
    if rustc_minor_ver >= 15 || rustc_dep_of_std {
        set_cfg("libc_priv_mod_use");
    }

    // Rust >= 1.19 supports unions:
    if rustc_minor_ver >= 19 || rustc_dep_of_std {
        set_cfg("libc_union");
    }

    // Rust >= 1.24 supports const mem::size_of:
    if rustc_minor_ver >= 24 || rustc_dep_of_std {
        set_cfg("libc_const_size_of");
    }

    // Rust >= 1.25 supports repr(align):
    if rustc_minor_ver >= 25 || rustc_dep_of_std || align_cargo_feature {
        set_cfg("libc_align");
    }

    // Rust >= 1.26 supports i128 and u128:
    if rustc_minor_ver >= 26 || rustc_dep_of_std {
        set_cfg("libc_int128");
    }

    // Rust >= 1.30 supports `core::ffi::c_void`, so libc can just re-export it.
    // Otherwise, it defines an incompatible type to retaining
    // backwards-compatibility.
    if rustc_minor_ver >= 30 || rustc_dep_of_std {
        set_cfg("libc_core_cvoid");
    }

    // Rust >= 1.33 supports repr(packed(N)) and cfg(target_vendor).
    if rustc_minor_ver >= 33 || rustc_dep_of_std {
        set_cfg("libc_packedN");
        set_cfg("libc_cfg_target_vendor");
    }

    // Rust >= 1.40 supports #[non_exhaustive].
    if rustc_minor_ver >= 40 || rustc_dep_of_std {
        set_cfg("libc_non_exhaustive");
    }

    // Rust >= 1.47 supports long array:
    if rustc_minor_ver >= 47 || rustc_dep_of_std {
        set_cfg("libc_long_array");
    }

    if rustc_minor_ver >= 51 || rustc_dep_of_std {
        set_cfg("libc_ptr_addr_of");
    }

    // Rust >= 1.37.0 allows underscores as anonymous constant names.
    if rustc_minor_ver >= 37 || rustc_dep_of_std {
        set_cfg("libc_underscore_const_names");
    }

    // #[thread_local] is currently unstable
    if rustc_dep_of_std {
        set_cfg("libc_thread_local");
    }

    // Rust >= 1.62.0 allows to use `const_extern_fn` for "Rust" and "C".
    if rustc_minor_ver >= 62 {
        set_cfg("libc_const_extern_fn");
    } else {
        // Rust < 1.62.0 requires a crate feature and feature gate.
        if const_extern_fn_cargo_feature {
            if !is_nightly || rustc_minor_ver < 40 {
                panic!("const-extern-fn requires a nightly compiler >= 1.40");
            }
            set_cfg("libc_const_extern_fn_unstable");
            set_cfg("libc_const_extern_fn");
        }
    }

    // check-cfg is a nightly cargo/rustc feature to warn when unknown cfgs are used across the
    // codebase. libc can configure it if the appropriate environment variable is passed. Since
    // rust-lang/rust enforces it, this is useful when using a custom libc fork there.
    //
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#check-cfg
    if libc_check_cfg {
        for cfg in ALLOWED_CFGS {
            if rustc_minor_ver >= 75 {
                println!("cargo:rustc-check-cfg=cfg({})", cfg);
            } else {
                println!("cargo:rustc-check-cfg=values({})", cfg);
            }
        }
        for &(name, values) in CHECK_CFG_EXTRA {
            let values = values.join("\",\"");
            if rustc_minor_ver >= 75 {
                println!("cargo:rustc-check-cfg=cfg({},values(\"{}\"))", name, values);
            } else {
                println!("cargo:rustc-check-cfg=values({},\"{}\")", name, values);
            }
        }
    }
}

/// Run `rustc --version` and capture the output, adjusting arguments as needed if `clippy-driver`
/// is used instead.
fn rustc_version_cmd(is_clippy_driver: bool) -> Output {
    let rustc = env::var_os("RUSTC").expect("Failed to get rustc version: missing RUSTC env");

    let mut cmd = match env::var_os("RUSTC_WRAPPER") {
        Some(ref wrapper) if wrapper.is_empty() => Command::new(rustc),
        Some(wrapper) => {
            let mut cmd = Command::new(wrapper);
            cmd.arg(rustc);
            if is_clippy_driver {
                cmd.arg("--rustc");
            }

            cmd
        }
        None => Command::new(rustc),
    };

    cmd.arg("--version");

    let output = cmd.output().ok().expect("Failed to get rustc version");

    if !output.status.success() {
        panic!(
            "failed to run rustc: {}",
            String::from_utf8_lossy(output.stderr.as_slice())
        );
    }

    output
}

/// Return the minor version of `rustc`, as well as a bool indicating whether or not the version
/// is a nightly.
fn rustc_minor_nightly() -> (u32, bool) {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => panic!("Failed to get rustc version"),
            }
        };
    }

    let mut output = rustc_version_cmd(false);

    if otry!(str::from_utf8(&output.stdout).ok()).starts_with("clippy") {
        output = rustc_version_cmd(true);
    }

    let version = otry!(str::from_utf8(&output.stdout).ok());

    let mut pieces = version.split('.');

    if pieces.next() != Some("rustc 1") {
        panic!("Failed to get rustc version");
    }

    let minor = pieces.next();

    // If `rustc` was built from a tarball, its version string
    // will have neither a git hash nor a commit date
    // (e.g. "rustc 1.39.0"). Treat this case as non-nightly,
    // since a nightly build should either come from CI
    // or a git checkout
    let nightly_raw = otry!(pieces.next()).split('-').nth(1);
    let nightly = nightly_raw
        .map(|raw| raw.starts_with("dev") || raw.starts_with("nightly"))
        .unwrap_or(false);
    let minor = otry!(otry!(minor).parse().ok());

    (minor, nightly)
}

fn which_freebsd() -> Option<i32> {
    let output = std::process::Command::new("freebsd-version").output().ok();
    if output.is_none() {
        return None;
    }
    let output = output.unwrap();
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok();
    if stdout.is_none() {
        return None;
    }
    let stdout = stdout.unwrap();

    match &stdout {
        s if s.starts_with("10") => Some(10),
        s if s.starts_with("11") => Some(11),
        s if s.starts_with("12") => Some(12),
        s if s.starts_with("13") => Some(13),
        s if s.starts_with("14") => Some(14),
        s if s.starts_with("15") => Some(15),
        _ => None,
    }
}

fn emcc_version_code() -> Option<u64> {
    let output = std::process::Command::new("emcc")
        .arg("-dumpversion")
        .output()
        .ok();
    if output.is_none() {
        return None;
    }
    let output = output.unwrap();
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok();
    if stdout.is_none() {
        return None;
    }
    let version = stdout.unwrap();

    // Some Emscripten versions come with `-git` attached, so split the
    // version string also on the `-` char.
    let mut pieces = version.trim().split(|c| c == '.' || c == '-');

    let major = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let minor = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let patch = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);

    Some(major * 10000 + minor * 100 + patch)
}

fn set_cfg(cfg: &str) {
    if !ALLOWED_CFGS.contains(&cfg) {
        panic!("trying to set cfg {}, but it is not in ALLOWED_CFGS", cfg);
    }
    println!("cargo:rustc-cfg={}", cfg);
}
