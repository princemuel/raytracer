//! # Errors
//! This module defines all the errors that can occur during ray tracing
//! operations.
use std::error::Error;
use std::fmt;

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
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Division by zero or near-zero value
    DivisionByZero { operation: String, value: f64 },

    /// Matrix is not invertible (determinant is zero)
    MatrixNotInvertible { matrix_type: String },

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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    /// No intersections found when one was expected
    NoIntersection {
        shape_type:    String,
        ray_origin:    [f64; 3],
        ray_direction: [f64; 3],
    },

    /// Invalid shape parameters
    InvalidShape {
        shape_type: String,
        parameter:  String,
        value:      String,
        reason:     String,
    },

    /// Transform stack overflow (too many nested transformations)
    TransformStackOverflow {
        current_depth: usize,
        max_depth:     usize,
    },

    /// Invalid UV coordinates for texture mapping
    InvalidUvCoordinates {
        u:          f64,
        v:          f64,
        shape_type: String,
    },
}

/// Material and shading errors
#[derive(Debug, Clone, PartialEq)]
pub enum ShadingError {
    /// Invalid material property
    InvalidMaterial {
        property:    String,
        value:       f64,
        valid_range: (f64, f64),
    },

    /// Pattern generation failed
    PatternError {
        pattern_type: String,
        reason:       String,
    },

    /// Lighting calculation failed
    LightingError {
        reason:         String,
        light_position: [f64; 3],
        surface_point:  [f64; 3],
    },

    /// Recursive reflection/refraction depth exceeded
    RecursionLimitExceeded {
        current_depth: usize,
        max_depth:     usize,
        ray_type:      String, // "reflection" or "refraction"
    },
}

/// Scene and world management errors
#[derive(Debug, Clone, PartialEq)]
pub enum WorldError {
    /// Scene contains no objects
    EmptyScene,

    /// Scene contains no lights
    NoLights,

    /// Too many objects in scene (performance limit)
    TooManyObjects { count: usize, max_count: usize },

    /// Object ID not found in scene
    ObjectNotFound { object_id: String },

    /// Light ID not found in scene
    LightNotFound { light_id: String },

    /// Scene hierarchy is invalid (circular references, etc.)
    InvalidHierarchy { reason: String },
}

/// File I/O and external resource errors
#[derive(Debug)]
pub enum IoError {
    /// File operation failed
    FileOperation {
        operation: String,
        filename:  String,
        source:    std::io::Error,
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
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Missing required configuration
    MissingConfig {
        config_key:  String,
        config_file: String,
    },

    /// Invalid configuration value
    InvalidConfig {
        config_key:    String,
        value:         String,
        expected_type: String,
    },

    /// Configuration file is corrupted or unreadable
    CorruptedConfig {
        config_file: String,
        reason:      String,
    },

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
            Self::Math(e) => write!(f, "Math error: {}", e),
            Self::Graphics(e) => write!(f, "Graphics error: {}", e),
            Self::Geometry(e) => write!(f, "Geometry error: {}", e),
            Self::Shading(e) => write!(f, "Shading error: {}", e),
            Self::World(e) => write!(f, "World error: {}", e),
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Config(e) => write!(f, "Configuration error: {}", e),
            Self::Other(error) => write!(f, "Error: {}", error),
        }
    }
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero { operation, value } => {
                write!(f, "Division by zero in {}: value was {}", operation, value)
            },
            Self::MatrixNotInvertible { matrix_type } => {
                write!(
                    f,
                    "{} matrix is not invertible (determinant is zero)",
                    matrix_type
                )
            },
            Self::InvalidVector { operation, vector } => {
                write!(
                    f,
                    "Invalid vector for {}: ({}, {}, {})",
                    operation, vector[0], vector[1], vector[2]
                )
            },
            Self::InvalidTransform {
                transform_type,
                reason,
            } => {
                write!(f, "Invalid {} transformation: {}", transform_type, reason)
            },
            Self::PrecisionError {
                operation,
                expected,
                actual,
                epsilon,
            } => {
                write!(
                    f,
                    "Precision error in {}: expected {}, got {} (ε={})",
                    operation, expected, actual, epsilon
                )
            },
        }
    }
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions {
                width,
                height,
                max_size,
            } => {
                write!(
                    f,
                    "Invalid canvas dimensions: {}x{} (max size: {})",
                    width, height, max_size
                )
            },
            Self::PixelOutOfBounds { x, y, width, height } => {
                write!(
                    f,
                    "Pixel ({}, {}) is out of bounds for {}x{} canvas",
                    x, y, width, height
                )
            },
            Self::InvalidColorValue {
                component,
                value,
                valid_range,
            } => {
                write!(
                    f,
                    "Invalid color {}: {} (valid range: {:.2} to {:.2})",
                    component, value, valid_range.0, valid_range.1
                )
            },
            Self::InvalidCamera { reason } => {
                write!(f, "Invalid camera configuration: {}", reason)
            },
            Self::RayCastError { reason } => {
                write!(f, "Ray casting failed: {}", reason)
            },
        }
    }
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoIntersection {
                shape_type,
                ray_origin,
                ray_direction,
            } => {
                write!(
                    f,
                    "No intersection with {} (ray: origin=({:.2}, {:.2}, {:.2}), dir=({:.2}, {:.2}, \
                     {:.2}))",
                    shape_type,
                    ray_origin[0],
                    ray_origin[1],
                    ray_origin[2],
                    ray_direction[0],
                    ray_direction[1],
                    ray_direction[2]
                )
            },
            Self::InvalidShape {
                shape_type,
                parameter,
                value,
                reason,
            } => {
                write!(f, "Invalid {} {}: {} ({})", shape_type, parameter, value, reason)
            },
            Self::TransformStackOverflow {
                current_depth,
                max_depth,
            } => {
                write!(
                    f,
                    "Transform stack overflow: {current_depth} levels (max: {max_depth})",
                )
            },
            Self::InvalidUvCoordinates { u, v, shape_type } => {
                write!(f, "Invalid UV coordinates ({}, {}) for {}", u, v, shape_type)
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
                valid_range,
            } => {
                write!(
                    f,
                    "Invalid material {}: {} (valid range: {:.2} to {:.2})",
                    property, value, valid_range.0, valid_range.1
                )
            },
            Self::PatternError { pattern_type, reason } => {
                write!(f, "Pattern generation failed for {}: {}", pattern_type, reason)
            },
            Self::LightingError {
                reason,
                light_position,
                surface_point,
            } => {
                write!(
                    f,
                    "Lighting calculation failed: {} (light: ({:.2}, {:.2}, {:.2}), surface: ({:.2}, \
                     {:.2}, {:.2}))",
                    reason,
                    light_position[0],
                    light_position[1],
                    light_position[2],
                    surface_point[0],
                    surface_point[1],
                    surface_point[2]
                )
            },
            Self::RecursionLimitExceeded {
                current_depth,
                max_depth,
                ray_type,
            } => {
                write!(
                    f,
                    "Recursion limit exceeded for {}: {} levels (max: {})",
                    ray_type, current_depth, max_depth
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
                write!(f, "Too many objects in scene: {} (max: {})", count, max_count)
            },
            Self::ObjectNotFound { object_id } => {
                write!(f, "Object not found: {}", object_id)
            },
            Self::LightNotFound { light_id } => {
                write!(f, "Light not found: {}", light_id)
            },
            Self::InvalidHierarchy { reason } => {
                write!(f, "Invalid scene hierarchy: {}", reason)
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
                write!(f, "Failed to {} file '{}': {}", operation, filename, source)
            },
            Self::UnsupportedFormat {
                filename,
                format,
                supported_formats,
            } => {
                write!(
                    f,
                    "Unsupported format '{}' for file '{}'. Supported: {}",
                    format,
                    filename,
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
            Self::MissingConfig {
                config_key,
                config_file,
            } => {
                write!(
                    f,
                    "Missing configuration '{}' in file '{}'",
                    config_key, config_file
                )
            },
            Self::InvalidConfig {
                config_key,
                value,
                expected_type,
            } => {
                write!(
                    f,
                    "Invalid configuration '{}': got '{}', expected {}",
                    config_key, value, expected_type
                )
            },
            Self::CorruptedConfig { config_file, reason } => {
                write!(f, "Corrupted configuration file '{}': {}", config_file, reason)
            },
            Self::VersionMismatch {
                required_version,
                found_version,
                component,
            } => {
                write!(
                    f,
                    "Version mismatch for {}: required {}, found {}",
                    component, required_version, found_version
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

impl From<std::io::Error> for TracerError {
    fn from(error: std::io::Error) -> Self {
        TracerError::Io(IoError::FileOperation {
            operation: "unknown".to_string(),
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
