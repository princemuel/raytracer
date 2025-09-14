//! # Errors
//! This module defines all the errors that can occur during ray tracing
//! operations.
use core::fmt;

/// The main error type for all ray tracer operations
#[derive(Debug)]
pub enum RayTraceError {
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

/// Convenient Result type alias
pub type Result<T> = core::result::Result<T, RayTraceError>;

// ================================
// Error Display Implementations
// ================================

impl fmt::Display for RayTraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RayTraceError::Math(e) => write!(f, "Math error: {}", e),
            RayTraceError::Graphics(e) => write!(f, "Graphics error: {}", e),
            RayTraceError::Geometry(e) => write!(f, "Geometry error: {}", e),
            RayTraceError::Shading(e) => write!(f, "Shading error: {}", e),
            RayTraceError::World(e) => write!(f, "World error: {}", e),
            RayTraceError::Io(e) => write!(f, "I/O error: {}", e),
            RayTraceError::Config(e) => write!(f, "Configuration error: {}", e),
            RayTraceError::Other(error) => write!(f, "Error: {}", error),
        }
    }
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero { operation, value } => {
                write!(f, "Division by zero in {}: value was {}", operation, value)
            },
            MathError::MatrixNotInvertible { matrix_type } => {
                write!(
                    f,
                    "{} matrix is not invertible (determinant is zero)",
                    matrix_type
                )
            },
            MathError::InvalidVector { operation, vector } => {
                write!(
                    f,
                    "Invalid vector for {}: ({}, {}, {})",
                    operation, vector[0], vector[1], vector[2]
                )
            },
            MathError::InvalidTransform {
                transform_type,
                reason,
            } => {
                write!(f, "Invalid {} transformation: {}", transform_type, reason)
            },
            MathError::PrecisionError {
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
            GraphicsError::InvalidDimensions {
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
            GraphicsError::PixelOutOfBounds { x, y, width, height } => {
                write!(
                    f,
                    "Pixel ({}, {}) is out of bounds for {}x{} canvas",
                    x, y, width, height
                )
            },
            GraphicsError::InvalidColorValue {
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
            GraphicsError::InvalidCamera { reason } => {
                write!(f, "Invalid camera configuration: {}", reason)
            },
            GraphicsError::RayCastError { reason } => {
                write!(f, "Ray casting failed: {}", reason)
            },
        }
    }
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::NoIntersection {
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
            GeometryError::InvalidShape {
                shape_type,
                parameter,
                value,
                reason,
            } => {
                write!(f, "Invalid {} {}: {} ({})", shape_type, parameter, value, reason)
            },
            GeometryError::TransformStackOverflow {
                current_depth,
                max_depth,
            } => {
                write!(
                    f,
                    "Transform stack overflow: {current_depth} levels (max: {max_depth})",
                )
            },
            GeometryError::InvalidUvCoordinates { u, v, shape_type } => {
                write!(f, "Invalid UV coordinates ({}, {}) for {}", u, v, shape_type)
            },
        }
    }
}

impl fmt::Display for ShadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShadingError::InvalidMaterial {
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
            ShadingError::PatternError { pattern_type, reason } => {
                write!(f, "Pattern generation failed for {}: {}", pattern_type, reason)
            },
            ShadingError::LightingError {
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
            ShadingError::RecursionLimitExceeded {
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
            WorldError::EmptyScene => write!(f, "Scene contains no objects to render"),
            WorldError::NoLights => write!(f, "Scene contains no lights"),
            WorldError::TooManyObjects { count, max_count } => {
                write!(f, "Too many objects in scene: {} (max: {})", count, max_count)
            },
            WorldError::ObjectNotFound { object_id } => {
                write!(f, "Object not found: {}", object_id)
            },
            WorldError::LightNotFound { light_id } => {
                write!(f, "Light not found: {}", light_id)
            },
            WorldError::InvalidHierarchy { reason } => {
                write!(f, "Invalid scene hierarchy: {}", reason)
            },
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::FileOperation {
                operation,
                filename,
                source,
            } => {
                write!(f, "Failed to {} file '{}': {}", operation, filename, source)
            },
            IoError::UnsupportedFormat {
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
            IoError::ParseError {
                filename,
                line_number,
                reason,
            } => match line_number {
                Some(line) => write!(f, "Parse error in '{}' at line {}: {}", filename, line, reason),
                None => write!(f, "Parse error in '{}': {}", filename, reason),
            },
            IoError::NetworkError { url, reason } => {
                write!(f, "Network error accessing '{}': {}", url, reason)
            },
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingConfig {
                config_key,
                config_file,
            } => {
                write!(
                    f,
                    "Missing configuration '{}' in file '{}'",
                    config_key, config_file
                )
            },
            ConfigError::InvalidConfig {
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
            ConfigError::CorruptedConfig { config_file, reason } => {
                write!(f, "Corrupted configuration file '{}': {}", config_file, reason)
            },
            ConfigError::VersionMismatch {
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
impl std::error::Error for MathError {}
impl std::error::Error for GraphicsError {}
impl std::error::Error for GeometryError {}
impl std::error::Error for ShadingError {}
impl std::error::Error for WorldError {}
impl std::error::Error for IoError {}
impl std::error::Error for ConfigError {}

impl From<std::io::Error> for RayTraceError {
    fn from(error: std::io::Error) -> Self {
        RayTraceError::Io(IoError::FileOperation {
            operation: "unknown".to_string(),
            filename:  "unknown".to_string(),
            source:    error,
        })
    }
}

impl From<MathError> for RayTraceError {
    fn from(error: MathError) -> Self { RayTraceError::Math(error) }
}

impl From<GraphicsError> for RayTraceError {
    fn from(error: GraphicsError) -> Self { RayTraceError::Graphics(error) }
}

impl From<GeometryError> for RayTraceError {
    fn from(error: GeometryError) -> Self { RayTraceError::Geometry(error) }
}

impl From<ShadingError> for RayTraceError {
    fn from(error: ShadingError) -> Self { RayTraceError::Shading(error) }
}

impl From<WorldError> for RayTraceError {
    fn from(error: WorldError) -> Self { RayTraceError::World(error) }
}

impl From<IoError> for RayTraceError {
    fn from(error: IoError) -> Self { RayTraceError::Io(error) }
}

impl From<ConfigError> for RayTraceError {
    fn from(error: ConfigError) -> Self { RayTraceError::Config(error) }
}

impl From<String> for RayTraceError {
    fn from(error: String) -> Self { RayTraceError::Other(error) }
}

impl From<&str> for RayTraceError {
    fn from(error: &str) -> Self { RayTraceError::Other(error.to_string()) }
}
