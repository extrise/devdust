//! Dev Dust Core Library
//!
//! This library provides functionality to detect various types of development projects
//! and clean their build artifacts to reclaim disk space.
//!
//! Supported project types:
//! - Rust (Cargo)
//! - Node.js/JavaScript
//! - Python
//! - .NET (C#/F#)
//! - Java (Maven, Gradle)
//! - Unity
//! - Unreal Engine
//! - And many more...

use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

// ============================================================================
// Project Type Definitions
// ============================================================================

/// Represents different types of development projects we can detect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    /// Rust projects (Cargo.toml)
    Rust,
    /// Node.js/JavaScript projects (package.json)
    Node,
    /// Python projects (.py files with common artifacts)
    Python,
    /// .NET projects (.csproj, .fsproj)
    DotNet,
    /// Unity game engine projects
    Unity,
    /// Unreal Engine projects (.uproject)
    Unreal,
    /// Java Maven projects (pom.xml)
    Maven,
    /// Java/Kotlin Gradle projects (build.gradle)
    Gradle,
    /// CMake projects (CMakeLists.txt)
    CMake,
    /// Haskell Stack projects (stack.yaml)
    HaskellStack,
    /// Scala SBT projects (build.sbt)
    ScalaSBT,
    /// PHP Composer projects (composer.json)
    Composer,
    /// Dart/Flutter projects (pubspec.yaml)
    Dart,
    /// Elixir projects (mix.exs)
    Elixir,
    /// Swift projects (Package.swift)
    Swift,
    /// Zig projects (build.zig)
    Zig,
    /// Godot 4.x projects (project.godot)
    Godot,
    /// Jupyter notebooks (.ipynb)
    Jupyter,
}

impl ProjectType {
    /// Returns the human-readable name of the project type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Node => "Node.js",
            Self::Python => "Python",
            Self::DotNet => ".NET",
            Self::Unity => "Unity",
            Self::Unreal => "Unreal Engine",
            Self::Maven => "Maven",
            Self::Gradle => "Gradle",
            Self::CMake => "CMake",
            Self::HaskellStack => "Haskell Stack",
            Self::ScalaSBT => "Scala SBT",
            Self::Composer => "PHP Composer",
            Self::Dart => "Dart/Flutter",
            Self::Elixir => "Elixir",
            Self::Swift => "Swift",
            Self::Zig => "Zig",
            Self::Godot => "Godot",
            Self::Jupyter => "Jupyter",
        }
    }

    /// Returns the directories that contain build artifacts for this project type
    pub fn artifact_directories(&self) -> &[&str] {
        match self {
            Self::Rust => &["target", ".xwin-cache"],
            Self::Node => &[
                "node_modules",
                ".next",
                ".nuxt",
                "dist",
                "build",
                ".angular",
            ],
            Self::Python => &[
                "__pycache__",
                ".pytest_cache",
                ".mypy_cache",
                ".ruff_cache",
                ".tox",
                ".nox",
                ".venv",
                "venv",
                ".hypothesis",
                "__pypackages__",
                "*.egg-info",
            ],
            Self::DotNet => &["bin", "obj"],
            Self::Unity => &[
                "Library",
                "Temp",
                "Obj",
                "Logs",
                "MemoryCaptures",
                "Build",
                "Builds",
            ],
            Self::Unreal => &[
                "Binaries",
                "Build",
                "Saved",
                "Intermediate",
                "DerivedDataCache",
            ],
            Self::Maven => &["target"],
            Self::Gradle => &["build", ".gradle"],
            Self::CMake => &["build", "cmake-build-debug", "cmake-build-release"],
            Self::HaskellStack => &[".stack-work"],
            Self::ScalaSBT => &["target", "project/target"],
            Self::Composer => &["vendor"],
            Self::Dart => &["build", ".dart_tool"],
            Self::Elixir => &["_build", ".elixir-tools", ".elixir_ls", ".lexical"],
            Self::Swift => &[".build", ".swiftpm"],
            Self::Zig => &["zig-cache", "zig-out"],
            Self::Godot => &[".godot"],
            Self::Jupyter => &[".ipynb_checkpoints"],
        }
    }

    /// Detects project type from a directory by checking for marker files
    pub fn detect_from_directory(path: &Path) -> Option<Self> {
        // Read directory entries
        let entries: Vec<_> = fs::read_dir(path).ok()?.filter_map(|e| e.ok()).collect();

        // Check for specific marker files
        for entry in &entries {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Check exact file names
            match file_name_str.as_ref() {
                "Cargo.toml" => return Some(Self::Rust),
                "package.json" => return Some(Self::Node),
                "pom.xml" => return Some(Self::Maven),
                "build.gradle" | "build.gradle.kts" => return Some(Self::Gradle),
                "CMakeLists.txt" => return Some(Self::CMake),
                "stack.yaml" => return Some(Self::HaskellStack),
                "build.sbt" => return Some(Self::ScalaSBT),
                "composer.json" => return Some(Self::Composer),
                "pubspec.yaml" => return Some(Self::Dart),
                "mix.exs" => return Some(Self::Elixir),
                "Package.swift" => return Some(Self::Swift),
                "build.zig" => return Some(Self::Zig),
                "project.godot" => return Some(Self::Godot),
                "Assembly-CSharp.csproj" => return Some(Self::Unity),
                _ => {}
            }

            // Check file extensions
            if file_name_str.ends_with(".uproject") {
                return Some(Self::Unreal);
            }
            if file_name_str.ends_with(".csproj") || file_name_str.ends_with(".fsproj") {
                // Distinguish between Unity, Godot, and regular .NET
                if Self::has_file(path, "project.godot") {
                    return Some(Self::Godot);
                } else if Self::has_file(path, "Assembly-CSharp.csproj") {
                    return Some(Self::Unity);
                } else {
                    return Some(Self::DotNet);
                }
            }
            if file_name_str.ends_with(".ipynb") {
                return Some(Self::Jupyter);
            }
            if file_name_str.ends_with(".py") {
                // Check if there are Python artifacts
                if Self::has_any_artifact(path, Self::Python.artifact_directories()) {
                    return Some(Self::Python);
                }
            }
        }

        None
    }

    /// Helper: Check if a directory contains a specific file
    fn has_file(dir: &Path, file_name: &str) -> bool {
        dir.join(file_name).exists()
    }

    /// Helper: Check if a directory contains any of the specified artifacts
    fn has_any_artifact(dir: &Path, artifacts: &[&str]) -> bool {
        artifacts.iter().any(|artifact| {
            let artifact_path = dir.join(artifact);
            artifact_path.exists()
        })
    }
}

// ============================================================================
// Project Structure
// ============================================================================

/// Represents a detected development project
#[derive(Debug, Clone)]
pub struct Project {
    /// The type of project detected
    pub project_type: ProjectType,
    /// The root path of the project
    pub path: PathBuf,
}

impl Project {
    /// Creates a new Project instance
    pub fn new(project_type: ProjectType, path: PathBuf) -> Self {
        Self { project_type, path }
    }

    /// Returns the display name of the project (usually the directory name)
    pub fn display_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    /// Calculates the total size of artifact directories in bytes
    pub fn calculate_artifact_size(&self, options: &ScanOptions) -> u64 {
        let mut total_size = 0u64;

        for artifact_dir in self.project_type.artifact_directories() {
            let artifact_path = self.path.join(artifact_dir);
            if artifact_path.exists() {
                total_size += calculate_directory_size(&artifact_path, options);
            }
        }

        total_size
    }

    /// Gets the last modified time of the project
    pub fn last_modified(&self, options: &ScanOptions) -> Result<SystemTime, std::io::Error> {
        let metadata = fs::metadata(&self.path)?;
        let mut most_recent = metadata.modified()?;

        // Walk through the project to find the most recent modification
        let walker = walkdir::WalkDir::new(&self.path)
            .follow_links(options.follow_symlinks)
            .same_file_system(options.same_filesystem);

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if modified > most_recent {
                        most_recent = modified;
                    }
                }
            }
        }

        Ok(most_recent)
    }

    /// Cleans (deletes) all artifact directories for this project
    pub fn clean(&self) -> Result<u64, CleanError> {
        let mut total_deleted = 0u64;
        let mut errors = Vec::new();

        for artifact_dir in self.project_type.artifact_directories() {
            let artifact_path = self.path.join(artifact_dir);

            if !artifact_path.exists() {
                continue;
            }

            // Calculate size before deletion
            let size = calculate_directory_size(&artifact_path, &ScanOptions::default());

            // Attempt to delete the directory
            match fs::remove_dir_all(&artifact_path) {
                Ok(_) => {
                    total_deleted += size;
                }
                Err(e) => {
                    errors.push((artifact_path.clone(), e));
                }
            }
        }

        if errors.is_empty() {
            Ok(total_deleted)
        } else {
            Err(CleanError::PartialFailure {
                deleted: total_deleted,
                errors,
            })
        }
    }
}

// ============================================================================
// Scanning Configuration
// ============================================================================

/// Options for scanning directories
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Whether to follow symbolic links
    pub follow_symlinks: bool,
    /// Whether to stay on the same filesystem
    pub same_filesystem: bool,
    /// Minimum age in seconds for projects to be included
    pub min_age_seconds: u64,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            follow_symlinks: false,
            same_filesystem: true,
            min_age_seconds: 0,
        }
    }
}

// ============================================================================
// Scanning Functions
// ============================================================================

/// Scans a directory recursively to find development projects
pub fn scan_directory<P: AsRef<Path>>(
    path: P,
    options: &ScanOptions,
) -> impl Iterator<Item = Result<Project, ScanError>> {
    let path = path.as_ref().to_path_buf();
    let options = options.clone();

    // Create a walkdir iterator with the specified options
    let walker = walkdir::WalkDir::new(&path)
        .follow_links(options.follow_symlinks)
        .same_file_system(options.same_filesystem)
        .into_iter();

    // Filter and map entries to projects
    walker.filter_map(move |entry| {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => return Some(Err(ScanError::WalkError(e))),
        };

        // Only process directories
        if !entry.file_type().is_dir() {
            return None;
        }

        // Skip hidden directories (starting with .)
        if entry.file_name().to_string_lossy().starts_with('.') {
            return None;
        }

        let dir_path = entry.path();

        // Try to detect project type
        if let Some(project_type) = ProjectType::detect_from_directory(dir_path) {
            let project = Project::new(project_type, dir_path.to_path_buf());

            // Check age filter if specified
            if options.min_age_seconds > 0 {
                if let Ok(last_modified) = project.last_modified(&options) {
                    if let Ok(elapsed) = last_modified.elapsed() {
                        if elapsed.as_secs() < options.min_age_seconds {
                            return None; // Too recent, skip
                        }
                    }
                }
            }

            return Some(Ok(project));
        }

        None
    })
}

/// Calculates the total size of a directory in bytes
pub fn calculate_directory_size<P: AsRef<Path>>(path: P, options: &ScanOptions) -> u64 {
    let walker = walkdir::WalkDir::new(path.as_ref())
        .follow_links(options.follow_symlinks)
        .same_file_system(options.same_filesystem);

    walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Formats a byte size into a human-readable string (e.g., "1.5 GB")
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f64 = bytes as f64;
    let unit_index = (bytes_f64.log(THRESHOLD).floor() as usize).min(UNITS.len() - 1);
    let size = bytes_f64 / THRESHOLD.powi(unit_index as i32);

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Formats elapsed time into a human-readable string (e.g., "2 days ago")
pub fn format_elapsed_time(seconds: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;
    const WEEK: u64 = DAY * 7;
    const MONTH: u64 = DAY * 30;
    const YEAR: u64 = DAY * 365;

    let (value, unit) = match seconds {
        s if s < MINUTE => (s, "second"),
        s if s < HOUR => (s / MINUTE, "minute"),
        s if s < DAY => (s / HOUR, "hour"),
        s if s < WEEK => (s / DAY, "day"),
        s if s < MONTH => (s / WEEK, "week"),
        s if s < YEAR => (s / MONTH, "month"),
        s => (s / YEAR, "year"),
    };

    let plural = if value == 1 { "" } else { "s" };
    format!("{} {}{} ago", value, unit, plural)
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during scanning
#[derive(Debug)]
pub enum ScanError {
    /// Error from walkdir
    WalkError(walkdir::Error),
    /// IO error
    IoError(std::io::Error),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WalkError(e) => write!(f, "Walk error: {}", e),
            Self::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for ScanError {}

impl From<walkdir::Error> for ScanError {
    fn from(e: walkdir::Error) -> Self {
        Self::WalkError(e)
    }
}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

/// Errors that can occur during cleaning
#[derive(Debug)]
pub enum CleanError {
    /// Complete failure to clean
    IoError(std::io::Error),
    /// Some directories were cleaned, but others failed
    PartialFailure {
        deleted: u64,
        errors: Vec<(PathBuf, std::io::Error)>,
    },
}

impl fmt::Display for CleanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "Clean error: {}", e),
            Self::PartialFailure { deleted, errors } => {
                write!(
                    f,
                    "Partially cleaned ({} bytes), {} errors occurred",
                    deleted,
                    errors.len()
                )
            }
        }
    }
}

impl Error for CleanError {}

impl From<std::io::Error> for CleanError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512.0 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1_048_576), "1.0 MB");
        assert_eq!(format_size(1_073_741_824), "1.0 GB");
    }

    #[test]
    fn test_format_elapsed_time() {
        assert_eq!(format_elapsed_time(0), "0 seconds ago");
        assert_eq!(format_elapsed_time(1), "1 second ago");
        assert_eq!(format_elapsed_time(59), "59 seconds ago");
        assert_eq!(format_elapsed_time(60), "1 minute ago");
        assert_eq!(format_elapsed_time(3600), "1 hour ago");
        assert_eq!(format_elapsed_time(86400), "1 day ago");
    }

    #[test]
    fn test_project_type_names() {
        assert_eq!(ProjectType::Rust.name(), "Rust");
        assert_eq!(ProjectType::Node.name(), "Node.js");
        assert_eq!(ProjectType::Python.name(), "Python");
    }
}
