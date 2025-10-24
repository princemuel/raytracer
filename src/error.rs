//! # Errors
//! This module defines all the errors that can occur during ray tracing
//! operations.
use std::error::Error;
use std::{fmt, io};

/// The main error type for all ray tracer operations
#[derive(Debug)]
pub enum TracerError {
    /// Mathematical operation errors
    Math(MathError),
    /// Canvas and image processing errors
    Graphics(GraphicsError),
    /// 3D geometry and intersection errors
    Geometry(GeometryError),
    /// Material and shading errors
    Shading(ShadingError),
    /// Scene and world management errors
    World(WorldError),
    /// File I/O errors
    Io(IoError),
    /// Configuration and parsing errors
    Config(ConfigError),
    /// Generic error with custom message
    Other(String),
}

/// Mathematical operation errors
#[derive(Clone, Debug, PartialEq)]
pub enum MathError {
    /// Division by zero or near-zero value
    DivisionByZero { operation: String, value: f64 },
    /// Matrix is not invertible (determinant is zero)
    MatrixNotInvertible { matrix: String },
    /// Invalid vector operation (e.g., normalizing zero vector)
    InvalidVector { operation: String, vector: [f64; 3] },
    /// Invalid transformation parameters
    InvalidTransform {
        transform_type: String,
        reason:         String,
    },
    /// Numerical precision issues
    PrecisionError {
        operation: String,
        expected:  f64,
        actual:    f64,
        epsilon:   f64,
    },
}

/// Canvas and rendering errors
#[derive(Clone, Debug, PartialEq)]
pub enum GraphicsError {
    /// Canvas dimensions are invalid
    InvalidDimensions {
        width:    usize,
        height:   usize,
        max_size: usize,
    },
    /// Pixel coordinates are out of bounds
    PixelOutOfBounds {
        x:      usize,
        y:      usize,
        width:  usize,
        height: usize,
    },
    /// Color values are out of valid range
    InvalidColorValue {
        component:   String,
        value:       f64,
        valid_range: (f64, f64),
    },
    /// Camera configuration is invalid
    InvalidCamera { reason: String },
    /// Ray casting failed
    RayCastError { reason: String },
}

/// Geometry and intersection errors
#[derive(Clone, Debug, PartialEq)]
pub enum GeometryError {
    /// No intersections found when one was expected
    NoIntersection {
        shape:     String,
        origin:    [f64; 3],
        direction: [f64; 3],
    },
    /// Invalid shape parameters
    InvalidShape {
        shape:     String,
        parameter: String,
        value:     String,
        reason:    String,
    },
    /// Transform stack overflow (too many nested transformations)
    TransformStackOverflow { depth: usize, max_depth: usize },
    /// Invalid UV coordinates for texture mapping
    InvalidUvCoordinates { u: f64, v: f64, shape: String },
}

/// Material and shading errors
#[derive(Clone, Debug, PartialEq)]
pub enum ShadingError {
    /// Invalid material property
    InvalidMaterial {
        property:    String,
        value:       f64,
        valid_range: (f64, f64),
    },
    /// Pattern generation failed
    PatternError { pattern: String, reason: String },
    /// Lighting calculation failed
    LightingError {
        reason:         String,
        light_position: [f64; 3],
        surface_point:  [f64; 3],
    },
    /// Recursive reflection/refraction depth exceeded
    RecursionLimitExceeded {
        depth:     usize,
        max_depth: usize,
        ray:       String, // "reflection" or "refraction"
    },
}

/// Scene and world management errors
#[derive(Clone, Debug, PartialEq)]
pub enum WorldError {
    /// Scene contains no objects
    EmptyScene,
    /// Scene contains no lights
    NoLights,
    /// Too many objects in scene (performance limit)
    TooManyObjects { count: usize, max_count: usize },
    /// Object ID not found in scene
    ObjectNotFound { id: String },
    /// Light ID not found in scene
    LightNotFound { id: String },
    /// Scene hierarchy is invalid (circular references, etc.)
    InvalidHierarchy { reason: String },
}

/// File I/O and external resource errors
#[derive(Debug)]
pub enum IoError {
    /// File operation failed
    FileOperation {
        operation: io::ErrorKind, // !NOTE: could be String. need to test
        filename:  String,
        source:    io::Error,
    },
    /// Image format not supported
    UnsupportedFormat {
        filename:          String,
        format:            String,
        supported_formats: Vec<String>,
    },
    /// File parsing failed
    ParseError {
        filename:    String,
        line_number: Option<usize>,
        reason:      String,
    },
    /// Network resource unavailable (for future web features)
    NetworkError { url: String, reason: String },
}

/// Configuration and setup errors
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigError {
    /// Missing required configuration
    MissingConfig { key: String, file: String },
    /// Invalid configuration value
    InvalidConfig {
        key:      String,
        value:    String,
        expected: String,
    },

    /// Configuration file is corrupted or unreadable
    CorruptedConfig { file: String, reason: String },
    /// Version incompatibility
    VersionMismatch {
        required_version: String,
        found_version:    String,
        component:        String,
    },
}

// ================================
// Error Display Implementations
// ================================

impl fmt::Display for TracerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Math(e) => write!(f, "MathError: {e}",),
            Self::Graphics(e) => write!(f, "GraphicsError: {e}",),
            Self::Geometry(e) => write!(f, "GeometryError: {e}",),
            Self::Shading(e) => write!(f, "ShadingError: {e}",),
            Self::World(e) => write!(f, "WorldError: {e}",),
            Self::Io(e) => write!(f, "I/O Error: {e}",),
            Self::Config(e) => write!(f, "ConfigurationError: {e}",),
            Self::Other(e) => write!(f, "Error: {e}",),
        }
    }
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero { operation: op, value } => {
                write!(f, "Division by zero in {op}: value was {value}")
            },
            Self::MatrixNotInvertible { matrix } => {
                write!(f, "{matrix} matrix is not invertible (determinant is zero)",)
            },
            Self::InvalidVector {
                operation: op,
                vector: [x, y, z],
            } => {
                write!(f, "Invalid vector for {op}: ({x}, {y}, {z})",)
            },
            Self::InvalidTransform {
                transform_type,
                reason,
            } => {
                write!(f, "Invalid {transform_type} transformation: {reason}")
            },
            Self::PrecisionError {
                operation: op,
                expected,
                actual,
                epsilon,
            } => {
                write!(
                    f,
                    "Precision error in {op}: expected {expected}, got {actual} (ε={epsilon})",
                )
            },
        }
    }
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions {
                width: w,
                height: h,
                max_size: size,
            } => {
                write!(f, "Invalid canvas dimensions: {w}x{h} (max size: {size})",)
            },
            Self::PixelOutOfBounds {
                x,
                y,
                width: w,
                height: h,
            } => {
                write!(f, "Pixel ({x}, {y}) is out of bounds for {w}x{h} canvas",)
            },
            Self::InvalidColorValue {
                component,
                value,
                valid_range: (a, b),
            } => {
                write!(
                    f,
                    "Invalid color {component}: {value} (valid range: {a:.2} to {b:.2})",
                )
            },
            Self::InvalidCamera { reason } => {
                write!(f, "Invalid camera configuration: {reason}")
            },
            Self::RayCastError { reason } => {
                write!(f, "Ray casting failed: {reason}")
            },
        }
    }
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoIntersection {
                shape,
                origin: [x, y, z],
                direction: [a, b, c],
            } => {
                write!(
                    f,
                    "No intersection with {shape} (ray: origin=({x:.2}, {y:.2}, {z:.2}), \
                     direction=({a:.2}, {b:.2}, {c:.2}))",
                )
            },
            Self::InvalidShape {
                shape,
                parameter,
                value,
                reason,
            } => {
                write!(f, "Invalid {shape} {parameter}: {value} ({reason})")
            },
            Self::TransformStackOverflow { depth, max_depth } => {
                write!(f, "Transform stack overflow: {depth} levels (max: {max_depth})",)
            },
            Self::InvalidUvCoordinates { u, v, shape } => {
                write!(f, "Invalid UV coordinates ({u}, {v}) for {shape}")
            },
        }
    }
}

impl fmt::Display for ShadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMaterial {
                property,
                value,
                valid_range: (a, b),
            } => {
                write!(
                    f,
                    "Invalid material {property}: {value} (valid range: {a:.2} to {b:.2})",
                )
            },
            Self::PatternError { pattern, reason } => {
                write!(f, "Pattern generation failed for {pattern}: {reason}")
            },
            Self::LightingError {
                reason,
                light_position: [a, b, c],
                surface_point: [x, y, z],
            } => {
                write!(
                    f,
                    "Lighting calculation failed: {reason} (light: ({a:.2}, {b:.2}, {c:.2}),
                    surface: ({x:.2}, {y:.2}, {z:.2}))",
                )
            },
            Self::RecursionLimitExceeded {
                depth,
                max_depth,
                ray,
            } => {
                write!(
                    f,
                    "Recursion limit exceeded for {ray}: {depth} levels (max: {max_depth})",
                )
            },
        }
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScene => write!(f, "Scene contains no objects to render"),
            Self::NoLights => write!(f, "Scene contains no lights"),
            Self::TooManyObjects { count, max_count } => {
                write!(f, "Too many objects in scene: {count} (max: {max_count})")
            },
            Self::ObjectNotFound { id } => {
                write!(f, "Object not found: {id}",)
            },
            Self::LightNotFound { id } => {
                write!(f, "Light not found: {id}")
            },
            Self::InvalidHierarchy { reason } => {
                write!(f, "Invalid scene hierarchy: {reason}")
            },
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileOperation {
                operation,
                filename,
                source,
            } => {
                write!(f, "Failed to {operation} file '{filename}': {source}")
            },
            Self::UnsupportedFormat {
                filename,
                format,
                supported_formats,
            } => {
                write!(
                    f,
                    "Unsupported format '{format}' for file '{filename}'. Supported: {}",
                    supported_formats.join(", ")
                )
            },
            Self::ParseError {
                filename,
                line_number,
                reason,
            } => match line_number {
                Some(line) => write!(f, "Parse error in '{}' at line {}: {}", filename, line, reason),
                None => write!(f, "Parse error in '{}': {}", filename, reason),
            },
            Self::NetworkError { url, reason } => {
                write!(f, "Network error accessing '{}': {}", url, reason)
            },
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingConfig { key, file } => {
                write!(f, "Missing configuration '{key}' in file '{file}'",)
            },
            Self::InvalidConfig { key, value, expected } => {
                write!(
                    f,
                    "Invalid configuration '{key}': got '{value}', expected {expected}",
                )
            },
            Self::CorruptedConfig { file, reason } => {
                write!(f, "Corrupted configuration file '{file}': {reason}")
            },
            Self::VersionMismatch {
                component,
                required_version,
                found_version,
            } => {
                write!(
                    f,
                    "Version mismatch for {component}: required {required_version}, found {found_version}",
                )
            },
        }
    }
}

// ================================
// Error Conversion Implementations
// ================================
impl Error for MathError {}
impl Error for GraphicsError {}
impl Error for GeometryError {}
impl Error for ShadingError {}
impl Error for WorldError {}
impl Error for IoError {}
impl Error for ConfigError {}

impl From<io::Error> for TracerError {
    fn from(error: io::Error) -> Self {
        Self::Io(IoError::FileOperation {
            operation: error.kind(),
            filename:  "unknown".to_string(),
            source:    error,
        })
    }
}

impl From<MathError> for TracerError {
    fn from(e: MathError) -> Self { Self::Math(e) }
}

impl From<GraphicsError> for TracerError {
    fn from(e: GraphicsError) -> Self { Self::Graphics(e) }
}

impl From<GeometryError> for TracerError {
    fn from(e: GeometryError) -> Self { Self::Geometry(e) }
}

impl From<ShadingError> for TracerError {
    fn from(e: ShadingError) -> Self { Self::Shading(e) }
}

impl From<WorldError> for TracerError {
    fn from(e: WorldError) -> Self { Self::World(e) }
}

impl From<IoError> for TracerError {
    fn from(e: IoError) -> Self { Self::Io(e) }
}

impl From<ConfigError> for TracerError {
    fn from(e: ConfigError) -> Self { Self::Config(e) }
}

impl From<String> for TracerError {
    fn from(e: String) -> Self { Self::Other(e) }
}

impl From<&str> for TracerError {
    fn from(e: &str) -> Self { Self::Other(e.to_string()) }
}
