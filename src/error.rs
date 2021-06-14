use std::error::Error as StdError;
use std::fmt;

/// A global error enum that encapsulates other more specific
/// types of errors.
#[derive(Debug)]
pub enum Error {
    IllegalMetricName,
    InvalidTimeDuration,
    IllegalTimeSeriesSelector,
    Reqwest(reqwest::Error),
    ResponseError(ResponseError),
    UnsupportedResponseDataType(UnsupportedResponseDataType),
    UnknownResponseStatus(UnknownResponseStatus),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IllegalMetricName => IllegalMetricNameError.fmt(f),
            Self::InvalidTimeDuration => InvalidTimeDurationError.fmt(f),
            Self::IllegalTimeSeriesSelector => IllegalTimeSeriesSelectorError.fmt(f),
            Self::Reqwest(e) => e.fmt(f),
            Self::ResponseError(e) => e.fmt(f),
            Self::UnsupportedResponseDataType(e) => e.fmt(f),
            Self::UnknownResponseStatus(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

/// This error is thrown when a reserved PromQL keyword is used
/// as metric name in a `Selector`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalMetricNameError;

impl fmt::Display for IllegalMetricNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided metric name is a reserved PromQL keyword")
    }
}

/// This error is thrown when a time duration is invalidated or empty.
/// See the [Prometheus reference](https://prometheus.io/docs/prometheus/latest/querying/basics/#time-durations)
/// for the correct time duration syntax.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidTimeDurationError;

impl fmt::Display for InvalidTimeDurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the provided time duration is invalid as it does not comply with PromQL time duration syntax")
    }
}

/// This error is thrown when a `Selector` cannot be contructed from the
/// provided metric name and/or the list of labels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IllegalTimeSeriesSelectorError;

// error message was shamelessly copied from the PromQL documentation.
impl fmt::Display for IllegalTimeSeriesSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vector selectors must either specify a name or at least one label matcher that does not match the empty string")
    }
}

/// This error is thrown when the JSON response's "status" field contains "error".
/// The error-related information in the response is included in this error.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponseError {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "the JSON response contains an error of type {}: {}",
            self.kind, self.message
        )
    }
}

/// This error is thrown when the JSON response's "data.resultType" field indicates
/// an unsupported data format that is not expected for this type of request.
/// For instant and range queries this must be either "vector" or "matrix".
#[derive(Debug, Clone, PartialEq)]
pub struct UnsupportedResponseDataType(pub String);

impl fmt::Display for UnsupportedResponseDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let UnsupportedResponseDataType(data_type) = self;
        write!(f, "the API returned an unsupported type of data, is '{}', must be either 'vector' or 'matrix'", data_type)
    }
}

/// This error is thrown when the JSON response's "status" field contains an
/// unexpected value. As per the Prometheus reference this must be either "success" or "error".
#[derive(Debug, Clone, PartialEq)]
pub struct UnknownResponseStatus(pub String);

impl fmt::Display for UnknownResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let UnknownResponseStatus(status) = self;
        write!(f, "the API returned an unknown response status , is '{}', must be either 'success' or 'error'", status)
    }
}
