//! Iron's HTTP Request representation and associated methods.

use std::io::{self, Read};
use std::net::SocketAddr;
use std::fmt::{self, Debug};

use http::Version as HttpVersion;

use typemap::TypeMap;
use plugin::Extensible;

pub use hyper::Request as HttpRequest;

#[cfg(test)]
use std::net::ToSocketAddrs;

pub use self::url::Url;
pub use hyper::body::Body;

use {Method, Protocol, Plugin, headers, Set};

mod url;

/// The `Request` given to all `Middleware`.
///
/// Stores all the properties of the client's request plus
/// an `TypeMap` for data communication between middleware.
pub struct Request {
    /// The requested URL.
    pub url: Url,

    /// The local address of the request.
    pub local_addr: SocketAddr,

    /// The request headers.
    pub headers: headers::HeaderMap,

    /// The request body as a stream.
    pub body: Body,

    /// The request method.
    pub method: Method,

    /// Extensible storage for data passed between middleware.
    pub extensions: TypeMap,

    /// The version of the HTTP protocol used.
    pub version: HttpVersion,

    _p: (),
}

impl Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Request {{"));

        try!(writeln!(f, "    url: {:?}", self.url));
        try!(writeln!(f, "    method: {:?}", self.method));
        try!(writeln!(f, "    local_addr: {:?}", self.local_addr));

        try!(write!(f, "}}"));
        Ok(())
    }
}

impl Request {
    /// Create a request from an HttpRequest.
    ///
    /// This constructor consumes the HttpRequest.
    pub fn from_http(req: HttpRequest<Body>, local_addr: SocketAddr, protocol: &Protocol)
                     -> Result<Request, String> {

        let headers = req.headers();
        let body = req.body();
        let method = req.method();
        let version = req.version();

        // let url = match uri {
        //     AbsoluteUri(ref url) => {
        //         match Url::from_generic_url(url.clone()) {
        //             Ok(url) => url,
        //             Err(e) => return Err(e)
        //         }
        //     },

        //     AbsolutePath(ref path) => {
        //         let url_string = match (version, headers.get::<headers::Host>()) {
        //             (_, Some(host)) => {
        //                 // Attempt to prepend the Host header (mandatory in HTTP/1.1)
        //                 if let Some(port) = host.port {
        //                     format!("{}://{}:{}{}", protocol.name(), host.hostname, port, path)
        //                 } else {
        //                     format!("{}://{}{}", protocol.name(), host.hostname, path)
        //                 }
        //             },
        //             (v, None) if v < HttpVersion::Http11 => {
        //                 // Attempt to use the local address? (host header is not required in HTTP/1.0).
        //                 match local_addr {
        //                     SocketAddr::V4(addr4) => format!("{}://{}:{}{}", protocol.name(), addr4.ip(), local_addr.port(), path),
        //                     SocketAddr::V6(addr6) => format!("{}://[{}]:{}{}", protocol.name(), addr6.ip(), local_addr.port(), path),
        //                 }
        //             },
        //             (_, None) => {
        //                 return Err("No host specified in request".into())
        //             }
        //         };

        //         match Url::parse(&url_string) {
        //             Ok(url) => url,
        //             Err(e) => return Err(format!("Couldn't parse requested URL: {}", e))
        //         }
        //     },
        //     _ => return Err("Unsupported request URI".into())
        // };

        let url = match Url::parse(&req.uri().to_string()) {
            Ok(url) => url,
            Err(e) => return Err(e)
        };

        Ok(Request {
            url: url,
            local_addr: local_addr,
            headers: *headers,
            body: *body,
            method: *method,
            extensions: TypeMap::new(),
            version: version,
            _p: (),
        })
    }

    #[cfg(test)]
    pub fn stub() -> Request {
        Request {
            url: Url::parse("http://www.rust-lang.org").unwrap(),
            remote_addr: "localhost:3000".to_socket_addrs().unwrap().next().unwrap(),
            local_addr: "localhost:3000".to_socket_addrs().unwrap().next().unwrap(),
            headers: Headers::new(),
            body: unsafe { ::std::mem::uninitialized() }, // FIXME(reem): Ugh
            method: Method::Get,
            extensions: TypeMap::new(),
            version: HttpVersion::Http11,
            _p: (),
        }
    }
}

// /// The body of an Iron request,
// pub struct Body<'a, 'b: 'a>(HttpReader<&'a mut buffer::BufReader<&'b mut NetworkStream>>);

// impl Body {
//     /// Create a new reader for use in an Iron request from a hyper HttpReader.
//     pub fn new(reader: HttpReader<&'a mut buffer::BufReader<&'b mut NetworkStream>>) -> Body {
//         Body(reader)
//     }
// }

// impl Read for Body {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         self.0.read(buf)
//     }
// }

// Allow plugins to attach to requests.
impl Extensible for Request {
    fn extensions(&self) -> &TypeMap {
        &self.extensions
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.extensions
    }
}

impl<'a, 'b> Plugin for Request {}
impl Set for Request {}
