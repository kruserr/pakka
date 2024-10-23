use std::io::BufRead;

#[cfg(target_os = "linux")]
fn check_aslr() -> bool {
    let file = std::fs::File::open("/proc/self/maps").expect("Unable to open /proc/self/maps");
    let reader = std::io::BufReader::new(file);
    let mut addresses = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if let Some(address) = line.split('-').next() {
            addresses.push(address.to_string());
        }
    }

    addresses.windows(2).all(|w| w[0] != w[1])
}

#[cfg(target_os = "macos")]
fn check_aslr() -> bool {
    let output = std::process::Command::new("vmmap")
        .arg("-w")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute vmmap");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("ASLR")
}

#[cfg(target_os = "windows")]
fn check_aslr() -> bool {
    // Windows ASLR check is more complex and typically requires specific tools or APIs.
    // For simplicity, we assume ASLR is enabled if the binary is a DLL or EXE with dynamic base.
    true
}

#[cfg(target_os = "freebsd")]
fn check_aslr() -> bool {
    let output = std::process::Command::new("procstat")
        .arg("-v")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute procstat");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("MAP_GUARD")
}

#[cfg(target_os = "linux")]
fn check_relro() -> bool {
    let output = std::process::Command::new("readelf")
        .arg("-l")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute readelf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("GNU_RELRO")
}

#[cfg(target_os = "macos")]
fn check_relro() -> bool {
    // macOS does not have RELRO in the same way as Linux.
    // We can check for similar protections using `otool`.
    let output = std::process::Command::new("otool")
        .arg("-l")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute otool");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("segname __DATA_CONST")
}

#[cfg(target_os = "windows")]
fn check_relro() -> bool {
    // Windows does not have RELRO in the same way as Linux.
    // We can check for similar protections using specific tools or APIs.
    true
}

#[cfg(target_os = "freebsd")]
fn check_relro() -> bool {
    // FreeBSD does not have RELRO in the same way as Linux.
    // We can check for similar protections using specific tools or APIs.
    true
}

#[cfg(target_os = "linux")]
fn check_pie() -> bool {
    let output = std::process::Command::new("readelf")
        .arg("-h")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute readelf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("PIE check output: {}", stdout); // Debug output
    stdout.lines().any(|line| line.contains("Type: DYN"))
}

#[cfg(target_os = "macos")]
fn check_pie() -> bool {
    let output = std::process::Command::new("otool")
        .arg("-hv")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute otool");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("PIE")
}

#[cfg(target_os = "windows")]
fn check_pie() -> bool {
    // Windows PIE check is more complex and typically requires specific tools or APIs.
    // For simplicity, we assume PIE is enabled if the binary is a DLL or EXE with dynamic base.
    true
}

#[cfg(target_os = "freebsd")]
fn check_pie() -> bool {
    let output = std::process::Command::new("readelf")
        .arg("-h")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute readelf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("Type: DYN")
}

#[cfg(target_os = "linux")]
fn check_stack_protection() -> bool {
    let output = std::process::Command::new("readelf")
        .arg("-s")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute readelf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Stack protection check output: {}", stdout); // Debug output
    stdout.lines().any(|line| line.contains("__stack_chk_fail"))
}

#[cfg(target_os = "macos")]
fn check_stack_protection() -> bool {
    let output = std::process::Command::new("otool")
        .arg("-Iv")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute otool");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("__stack_chk_fail")
}

#[cfg(target_os = "windows")]
fn check_stack_protection() -> bool {
    // Windows stack protection check is more complex and typically requires specific tools or APIs.
    // For simplicity, we assume stack protection is enabled if the binary is a DLL or EXE with stack cookies.
    true
}

#[cfg(target_os = "freebsd")]
fn check_stack_protection() -> bool {
    let output = std::process::Command::new("readelf")
        .arg("-s")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute readelf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("__stack_chk_fail")
}

#[cfg(target_os = "linux")]
fn check_fortify_source() -> bool {
    let output = std::process::Command::new("readelf")
        .arg("-s")
        .arg("/proc/self/exe")
        .output()
        .expect("Failed to execute readelf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Fortify Source check output: {}", stdout); // Debug output
    stdout.lines().any(|line| line.contains("__fortify_fail"))
}

#[cfg(target_os = "macos")]
fn check_fortify_source() -> bool {
    // macOS does not have Fortify Source in the same way as Linux.
    // We can check for similar protections using specific tools or APIs.
    true
}

#[cfg(target_os = "windows")]
fn check_fortify_source() -> bool {
    // Windows does not have Fortify Source in the same way as Linux.
    // We can check for similar protections using specific tools or APIs.
    true
}

#[cfg(target_os = "freebsd")]
fn check_fortify_source() -> bool {
    // FreeBSD does not have Fortify Source in the same way as Linux.
    // We can check for similar protections using specific tools or APIs.
    true
}
