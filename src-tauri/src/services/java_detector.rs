use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaInfo {
    pub path: String,
    pub version: String,
    pub vendor: String,
    pub is_64bit: bool,
    pub major_version: u32,
}

pub fn detect_java_installations() -> Vec<JavaInfo> {
    let mut results = Vec::new();
    let candidate_paths = get_candidate_paths();

    for path in candidate_paths {
        if let Some(info) = check_java(&path) {
            if !results.iter().any(|r: &JavaInfo| r.path == info.path) {
                results.push(info);
            }
        }
    }

    // Sort: highest version first
    results.sort_by(|a, b| b.major_version.cmp(&a.major_version));

    results
}

fn get_candidate_paths() -> Vec<String> {
    let mut paths = Vec::new();

    // 1. java in system PATH
    paths.push("java".to_string());

    // 2. JAVA_HOME
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        push_java_exe(&java_home, &mut paths);
    }

    #[cfg(target_os = "windows")]
    {
        // 3. Scan ALL drive letters
        for letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", letter as char);
            if !std::path::Path::new(&drive).exists() {
                continue;
            }

            // Standard install locations
            let base_dirs = vec![
                format!("{}Program Files\\Java", drive),
                format!("{}Program Files (x86)\\Java", drive),
                format!("{}Program Files\\Eclipse Adoptium", drive),
                format!("{}Program Files\\AdoptOpenJDK", drive),
                format!("{}Program Files\\Zulu", drive),
                format!("{}Program Files\\BellSoft", drive),
                format!("{}Program Files\\Amazon Corretto", drive),
                format!("{}Program Files\\Microsoft", drive),
                format!("{}Program Files\\Common Files\\Oracle\\Java\\javapath", drive),
                format!("{}Program Files\\GraalVM", drive),
            ];

            for dir in &base_dirs {
                deep_scan_java_dir(dir, &mut paths, 2);
            }

            // Custom locations people commonly use
            let custom_roots = vec![
                format!("{}Java", drive),
                format!("{}java", drive),
                format!("{}jdk", drive),
                format!("{}jre", drive),
            ];

            for dir in &custom_roots {
                deep_scan_java_dir(dir, &mut paths, 3);
            }

            // Game-related Java installations (like Minecraft bundled Java)
            let game_dirs = vec![
                format!("{}Game", drive),
                format!("{}Games", drive),
                format!("{}game", drive),
                format!("{}games", drive),
            ];

            for dir in &game_dirs {
                scan_for_java_recursive(dir, &mut paths, 4);
            }
        }

        // 4. User profile locations
        if let Ok(user) = std::env::var("USERPROFILE") {
            let user_dirs = vec![
                // .minecraft bundled runtimes (like PCL detects)
                format!("{}\\AppData\\Roaming\\.minecraft\\runtime", user),
                // Scoop
                format!("{}\\scoop\\apps", user),
                // IntelliJ / Android Studio bundled JDKs
                format!("{}\\.jdks", user),
                // Gradle
                format!("{}\\.gradle\\jdks", user),
                // sdkman
                format!("{}\\.sdkman\\candidates\\java", user),
                // Local programs
                format!("{}\\AppData\\Local\\Programs", user),
            ];

            for dir in &user_dirs {
                deep_scan_java_dir(dir, &mut paths, 4);
            }
        }

        // 5. APPDATA locations
        if let Ok(appdata) = std::env::var("APPDATA") {
            // .minecraft runtime
            let mc_runtime = format!("{}\\..\\Roaming\\.minecraft\\runtime", appdata);
            deep_scan_java_dir(&mc_runtime, &mut paths, 4);
            let mc_runtime2 = format!("{}\\.minecraft\\runtime", appdata);
            deep_scan_java_dir(&mc_runtime2, &mut paths, 4);
        }

        // 6. Try Windows Registry via `where` command
        if let Ok(output) = Command::new("where").arg("java").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    paths.push(trimmed.to_string());
                }
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        paths.push("/usr/bin/java".to_string());
        paths.push("/usr/local/bin/java".to_string());

        let jvm_dirs = vec![
            "/usr/lib/jvm",
            "/usr/local/lib/jvm",
            "/Library/Java/JavaVirtualMachines",
        ];

        for dir in jvm_dirs {
            deep_scan_java_dir(dir, &mut paths, 3);
        }

        if let Ok(home) = std::env::var("HOME") {
            let home_dirs = vec![
                format!("{}/.jdks", home),
                format!("{}/.sdkman/candidates/java", home),
                format!("{}/.gradle/jdks", home),
            ];
            for dir in &home_dirs {
                deep_scan_java_dir(dir, &mut paths, 3);
            }
        }
    }

    paths
}

#[cfg(target_os = "windows")]
fn push_java_exe(dir: &str, paths: &mut Vec<String>) {
    let exe = format!("{}\\bin\\java.exe", dir);
    if std::path::Path::new(&exe).exists() {
        paths.push(exe);
    }
}

#[cfg(not(target_os = "windows"))]
fn push_java_exe(dir: &str, paths: &mut Vec<String>) {
    let exe = format!("{}/bin/java", dir);
    if std::path::Path::new(&exe).exists() {
        paths.push(exe);
    }
}

/// Scan a directory for java executables, searching subdirs up to `depth` levels
fn deep_scan_java_dir(dir: &str, paths: &mut Vec<String>, depth: u32) {
    if depth == 0 {
        return;
    }
    let dir_path = std::path::Path::new(dir);
    if !dir_path.exists() || !dir_path.is_dir() {
        return;
    }

    // Check bin/java.exe directly
    #[cfg(target_os = "windows")]
    {
        let java_exe = dir_path.join("bin").join("java.exe");
        if java_exe.exists() {
            paths.push(java_exe.to_string_lossy().to_string());
        }
        // Also check if java.exe is directly in this dir
        let java_direct = dir_path.join("java.exe");
        if java_direct.exists() {
            paths.push(java_direct.to_string_lossy().to_string());
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let java_exe = dir_path.join("bin").join("java");
        if java_exe.exists() {
            paths.push(java_exe.to_string_lossy().to_string());
        }
        // macOS: Contents/Home/bin/java
        let java_macos = dir_path.join("Contents").join("Home").join("bin").join("java");
        if java_macos.exists() {
            paths.push(java_macos.to_string_lossy().to_string());
        }
    }

    // Recurse into subdirectories
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                deep_scan_java_dir(
                    &entry_path.to_string_lossy(),
                    paths,
                    depth - 1,
                );
            }
        }
    }
}

/// Special recursive scan for game directories - looks for any java.exe
fn scan_for_java_recursive(dir: &str, paths: &mut Vec<String>, depth: u32) {
    if depth == 0 {
        return;
    }
    let dir_path = std::path::Path::new(dir);
    if !dir_path.exists() || !dir_path.is_dir() {
        return;
    }

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let dir_name = entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                // Only recurse into directories that might contain Java
                if dir_name.contains("java")
                    || dir_name.contains("jdk")
                    || dir_name.contains("jre")
                    || dir_name.contains("zulu")
                    || dir_name.contains("adoptium")
                    || dir_name.contains("corretto")
                    || dir_name.contains("graalvm")
                    || dir_name.contains("azul")
                    || dir_name.contains("minecraft")
                    || dir_name.contains("runtime")
                    || dir_name.contains("bin")
                {
                    deep_scan_java_dir(
                        &entry_path.to_string_lossy(),
                        paths,
                        depth - 1,
                    );
                    scan_for_java_recursive(
                        &entry_path.to_string_lossy(),
                        paths,
                        depth - 1,
                    );
                }
            }
        }
    }
}

fn check_java(path: &str) -> Option<JavaInfo> {
    let output = Command::new(path)
        .arg("-version")
        .output()
        .ok()?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let combined = if stderr.is_empty() { stdout } else { stderr };

    if combined.is_empty() {
        return None;
    }

    let version = extract_version(&combined).unwrap_or_else(|| "unknown".to_string());
    let vendor = extract_vendor(&combined).unwrap_or_else(|| "unknown".to_string());
    let is_64bit = combined.contains("64-Bit") || combined.contains("64-bit");
    let major = parse_major_version(&version);

    let resolved = resolve_java_path(path).unwrap_or_else(|| path.to_string());

    Some(JavaInfo {
        path: resolved,
        version,
        vendor,
        is_64bit,
        major_version: major,
    })
}

fn parse_major_version(version: &str) -> u32 {
    let first_part = version.split('.').next().unwrap_or("0");
    // Handle "1.8.0_xxx" format
    let major: u32 = first_part.parse().unwrap_or(0);
    if major == 1 {
        // Old format: 1.8.0 -> major is 8
        version
            .split('.')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(major)
    } else {
        major
    }
}

fn extract_version(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.contains("version") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    return Some(line[start + 1..start + 1 + end].to_string());
                }
            }
        }
    }
    None
}

fn extract_vendor(output: &str) -> Option<String> {
    let first_line = output.lines().next()?;
    // e.g. "openjdk version..." or "java version..."
    let vendor = first_line.split_whitespace().next()?;
    Some(vendor.to_string())
}

fn resolve_java_path(path: &str) -> Option<String> {
    // If it is already an absolute path, just return it
    let p = std::path::Path::new(path);
    if p.is_absolute() && p.exists() {
        return Some(path.to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("where").arg(path).output().ok()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.lines().next().map(|s| s.trim().to_string())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("which").arg(path).output().ok()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.lines().next().map(|s| s.trim().to_string())
    }
}

pub fn validate_java(path: &str) -> Result<JavaInfo, String> {
    check_java(path).ok_or_else(|| format!("Invalid Java path: {}", path))
}
