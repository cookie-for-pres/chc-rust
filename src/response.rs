use std::collections::HashMap;

pub enum StatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
    Unknown = 0,
}

pub enum ResponseType {
    TextPlain,
    TextHTML,
    ApplicationJSON,
    ApplicationXML,
    ApplicationJavascript,
    TextCSS,
    ImagePNG,
    AudioMPEG,
    VideoMP4,
    ApplicationOctetStream,
}

impl StatusCode {
    pub fn clone(&self) -> StatusCode {
        match self {
            StatusCode::Continue => StatusCode::Continue,
            StatusCode::SwitchingProtocols => StatusCode::SwitchingProtocols,
            StatusCode::Processing => StatusCode::Processing,
            StatusCode::EarlyHints => StatusCode::EarlyHints,
            StatusCode::Ok => StatusCode::Ok,
            StatusCode::Created => StatusCode::Created,
            StatusCode::Accepted => StatusCode::Accepted,
            StatusCode::NonAuthoritativeInformation => StatusCode::NonAuthoritativeInformation,
            StatusCode::NoContent => StatusCode::NoContent,
            StatusCode::ResetContent => StatusCode::ResetContent,
            StatusCode::PartialContent => StatusCode::PartialContent,
            StatusCode::MultiStatus => StatusCode::MultiStatus,
            StatusCode::AlreadyReported => StatusCode::AlreadyReported,
            StatusCode::IMUsed => StatusCode::IMUsed,
            StatusCode::MultipleChoices => StatusCode::MultipleChoices,
            StatusCode::MovedPermanently => StatusCode::MovedPermanently,
            StatusCode::Found => StatusCode::Found,
            StatusCode::SeeOther => StatusCode::SeeOther,
            StatusCode::NotModified => StatusCode::NotModified,
            StatusCode::UseProxy => StatusCode::UseProxy,
            StatusCode::TemporaryRedirect => StatusCode::TemporaryRedirect,
            StatusCode::PermanentRedirect => StatusCode::PermanentRedirect,
            StatusCode::BadRequest => StatusCode::BadRequest,
            StatusCode::Unauthorized => StatusCode::Unauthorized,
            StatusCode::PaymentRequired => StatusCode::PaymentRequired,
            StatusCode::Forbidden => StatusCode::Forbidden,
            StatusCode::NotFound => StatusCode::NotFound,
            StatusCode::MethodNotAllowed => StatusCode::MethodNotAllowed,
            StatusCode::NotAcceptable => StatusCode::NotAcceptable,
            StatusCode::ProxyAuthenticationRequired => StatusCode::ProxyAuthenticationRequired,
            StatusCode::RequestTimeout => StatusCode::RequestTimeout,
            StatusCode::Conflict => StatusCode::Conflict,
            StatusCode::Gone => StatusCode::Gone,
            StatusCode::LengthRequired => StatusCode::LengthRequired,
            StatusCode::PreconditionFailed => StatusCode::PreconditionFailed,
            StatusCode::PayloadTooLarge => StatusCode::PayloadTooLarge,
            StatusCode::UriTooLong => StatusCode::UriTooLong,
            StatusCode::UnsupportedMediaType => StatusCode::UnsupportedMediaType,
            StatusCode::RangeNotSatisfiable => StatusCode::RangeNotSatisfiable,
            StatusCode::ExpectationFailed => StatusCode::ExpectationFailed,
            StatusCode::ImATeapot => StatusCode::ImATeapot,
            StatusCode::MisdirectedRequest => StatusCode::MisdirectedRequest,
            StatusCode::UnprocessableEntity => StatusCode::UnprocessableEntity,
            StatusCode::Locked => StatusCode::Locked,
            StatusCode::FailedDependency => StatusCode::FailedDependency,
            StatusCode::UpgradeRequired => StatusCode::UpgradeRequired,
            StatusCode::PreconditionRequired => StatusCode::PreconditionRequired,
            StatusCode::TooManyRequests => StatusCode::TooManyRequests,
            StatusCode::RequestHeaderFieldsTooLarge => StatusCode::RequestHeaderFieldsTooLarge,
            StatusCode::UnavailableForLegalReasons => StatusCode::UnavailableForLegalReasons,
            StatusCode::InternalServerError => StatusCode::InternalServerError,
            StatusCode::NotImplemented => StatusCode::NotImplemented,
            StatusCode::BadGateway => StatusCode::BadGateway,
            StatusCode::ServiceUnavailable => StatusCode::ServiceUnavailable,
            StatusCode::GatewayTimeout => StatusCode::GatewayTimeout,
            StatusCode::HttpVersionNotSupported => StatusCode::HttpVersionNotSupported,
            StatusCode::VariantAlsoNegotiates => StatusCode::VariantAlsoNegotiates,
            StatusCode::InsufficientStorage => StatusCode::InsufficientStorage,
            StatusCode::LoopDetected => StatusCode::LoopDetected,
            StatusCode::NotExtended => StatusCode::NotExtended,
            StatusCode::NetworkAuthenticationRequired => StatusCode::NetworkAuthenticationRequired,
            StatusCode::Unknown => StatusCode::Unknown,
        }
    }
}

impl ResponseType {
    pub fn clone(&self) -> ResponseType {
        match self {
            ResponseType::TextPlain => ResponseType::TextPlain,
            ResponseType::TextHTML=> ResponseType::TextHTML,
            ResponseType::ApplicationJSON => ResponseType::ApplicationJSON,
            ResponseType::ApplicationXML => ResponseType::ApplicationXML,
            ResponseType::ApplicationJavascript => ResponseType::ApplicationJavascript,
            ResponseType::TextCSS => ResponseType::TextCSS,
            ResponseType::ImagePNG => ResponseType::ImagePNG,
            ResponseType::AudioMPEG => ResponseType::AudioMPEG,
            ResponseType::VideoMP4 => ResponseType::VideoMP4,
            ResponseType::ApplicationOctetStream => ResponseType::ApplicationOctetStream,
        }
    }
}

pub struct Response {
    pub response_type: ResponseType,
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(response_type: ResponseType, status_code: StatusCode) -> Response {
        Response {
            response_type,
            status_code,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn copy(&self) -> Response {
        Response {
            response_type: self.response_type.clone(),
            status_code: self.status_code.clone(),
            headers: self.headers.clone(),
            cookies: self.cookies.clone(),
            body: self.body.clone(),
        }
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn set_cookie(&mut self, key: String, value: String) {
        self.cookies.insert(key, value);
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn parse(&self) -> String {
        let mut response = String::new();
        match self.response_type {
            ResponseType::TextPlain => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }
                response.push_str("Content-Type: text/plain\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::TextHTML => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: text/html\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::ApplicationJSON => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: application/json\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::ImagePNG => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: image/png\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::AudioMPEG => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: audio/mpeg\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::VideoMP4 => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: video/mp4\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::ApplicationOctetStream => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: application/octet-stream\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::ApplicationXML => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: application/xml\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::ApplicationJavascript => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: application/javascript\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
            ResponseType::TextCSS => {
                let http = grfsc(self.status_code.clone());
                response.push_str(&http);
                response.push_str("\r\n");
                for (key, value) in self.headers.iter() {
                    response.push_str(&key);
                    response.push_str(": ");
                    response.push_str(&value);
                    response.push_str("\r\n");
                }
                if self.cookies.len() > 0 {
                    response.push_str("Set-Cookie: ");
                    for (key, value) in self.cookies.iter() {
                        response.push_str(&key);
                        response.push_str("=");
                        response.push_str(&value);
                        response.push_str(";\r\n");
                    }
                }

                response.push_str("Content-Type: text/css\r\n");
                response.push_str("\r\n");
                response.push_str(&self.body);
            }
        }

        response
    }
}

pub fn grfsc(sc: StatusCode) -> String {
    match sc {
        StatusCode::Continue => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Continue", status_code)
        }
        StatusCode::SwitchingProtocols => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Switching Protocols", status_code)
        }
        StatusCode::Processing => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Processing", status_code)
        }
        StatusCode::EarlyHints => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Early Hints", status_code)
        }
        StatusCode::Ok => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} OK", status_code)
        }
        StatusCode::Created => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Created", status_code)
        }
        StatusCode::Accepted => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Accepted", status_code)
        }
        StatusCode::NonAuthoritativeInformation => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Non-Authoritative Information", status_code)
        }
        StatusCode::NoContent => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} No Content", status_code)
        }
        StatusCode::ResetContent => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Reset Content", status_code)
        }
        StatusCode::PartialContent => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Partial Content", status_code)
        }
        StatusCode::MultiStatus => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Multi-Status", status_code)
        }
        StatusCode::AlreadyReported => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Already Reported", status_code)
        }
        StatusCode::IMUsed => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} IM Used", status_code)
        }
        StatusCode::MultipleChoices => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Multiple Choices", status_code)
        }
        StatusCode::MovedPermanently => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Moved Permanently", status_code)
        }
        StatusCode::Found => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Found", status_code)
        }
        StatusCode::SeeOther => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} See Other", status_code)
        }
        StatusCode::NotModified => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Not Modified", status_code)
        }
        StatusCode::UseProxy => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Use Proxy", status_code)
        }
        StatusCode::TemporaryRedirect => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Temporary Redirect", status_code)
        }
        StatusCode::PermanentRedirect => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Permanent Redirect", status_code)
        }
        StatusCode::BadRequest => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Bad Request", status_code)
        }
        StatusCode::Unauthorized => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Unauthorized", status_code)
        }
        StatusCode::PaymentRequired => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Payment Required", status_code)
        }
        StatusCode::Forbidden => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Forbidden", status_code)
        }
        StatusCode::NotFound => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Not Found", status_code)
        }
        StatusCode::MethodNotAllowed => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Method Not Allowed", status_code)
        }
        StatusCode::NotAcceptable => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Not Acceptable", status_code)
        }
        StatusCode::ProxyAuthenticationRequired => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Proxy Authentication Required", status_code)
        }
        StatusCode::RequestTimeout => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Request Timeout", status_code)
        }
        StatusCode::Conflict => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Conflict", status_code)
        }
        StatusCode::Gone => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Gone", status_code)
        }
        StatusCode::LengthRequired => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Length Required", status_code)
        }
        StatusCode::PreconditionFailed => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Precondition Failed", status_code)
        }
        StatusCode::PayloadTooLarge => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Payload Too Large", status_code)
        }
        StatusCode::UriTooLong => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} URI Too Long", status_code)
        }
        StatusCode::UnsupportedMediaType => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Unsupported Media Type", status_code)
        }
        StatusCode::RangeNotSatisfiable => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Range Not Satisfiable", status_code)
        }
        StatusCode::ExpectationFailed => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Expectation Failed", status_code)
        }
        StatusCode::ImATeapot => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} I'm a teapot", status_code)
        }
        StatusCode::MisdirectedRequest => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Misdirected Request", status_code)
        }
        StatusCode::UnprocessableEntity => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Unprocessable Entity", status_code)
        }
        StatusCode::Locked => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Locked", status_code)
        }
        StatusCode::FailedDependency => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Failed Dependency", status_code)
        }
        StatusCode::UpgradeRequired => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Upgrade Required", status_code)
        }
        StatusCode::PreconditionRequired => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Precondition Required", status_code)
        }
        StatusCode::TooManyRequests => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Too Many Requests", status_code)
        }
        StatusCode::RequestHeaderFieldsTooLarge => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Request Header Fields Too Large", status_code)
        }
        StatusCode::UnavailableForLegalReasons => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Unavailable For Legal Reasons", status_code)
        }
        StatusCode::InternalServerError => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Internal Server Error", status_code)
        }
        StatusCode::NotImplemented => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Not Implemented", status_code)
        }
        StatusCode::BadGateway => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Bad Gateway", status_code)
        }
        StatusCode::ServiceUnavailable => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Service Unavailable", status_code)
        }
        StatusCode::GatewayTimeout => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Gateway Timeout", status_code)
        }
        StatusCode::HttpVersionNotSupported => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} HTTP Version Not Supported", status_code)
        }
        StatusCode::VariantAlsoNegotiates => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Variant Also Negotiates", status_code)
        }
        StatusCode::InsufficientStorage => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Insufficient Storage", status_code)
        }
        StatusCode::LoopDetected => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Loop Detected", status_code)
        }
        StatusCode::NotExtended => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Not Extended", status_code)
        }
        StatusCode::NetworkAuthenticationRequired => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Network Authentication Required", status_code)
        }
        StatusCode::Unknown => {
            let status_code = sc as u32;
            format!("HTTP/1.1 {} Unknown", status_code)
        }
    }
}
