use super::postman;

///The initial code for AsCurl was copied over from https://gitlab.com/mcarton/dbg_as_curl
/// A wrapper around a request that displays as a cURL command.
pub struct AsCurl {
    req: reqwest::Request,
    raw_body: Option<String>,
    compress: bool,
    verbose: bool,
}

#[allow(dead_code)]
impl AsCurl {
    /// Construct an instance of `AsCurl` with the given request.
    pub fn new(postman_req: postman::Request) -> Result<AsCurl, String> {
        let raw_body = match &postman_req {
            //TODO why is it's type an owned one rather than a
            //reference ?
            postman::Request::String(_) => None,
            postman::Request::RequestStruct(req_struct) => {
                req_struct.body.as_ref().and_then(|b| b.raw.clone())
            }
        };
        let (_client, reqw) = postman_req.to_reqwest()?;
        Ok(Self {
            req: reqw,
            raw_body,
            compress: false,
            verbose: false,
        })
    }

    /// Adds '--compress' to the command line.
    pub fn compress(self) -> Self {
        Self {
            compress: true,
            ..self
        }
    }

    /// Adds '--verbose' to the command line.
    pub fn verbose(self) -> Self {
        Self {
            verbose: true,
            ..self
        }
    }
}

impl std::fmt::Debug for AsCurl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::fmt::Display for AsCurl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let AsCurl {
            req,
            raw_body,
            compress,
            verbose,
        } = self;

        write!(f, "curl ")?;

        if *compress {
            write!(f, "--compress ")?;
        }
        if *verbose {
            write!(f, "--verbose ")?;
        }

        let method = req.method();
        if method != "GET" {
            write!(f, "-X {} ", method)?;
        }

        for (name, value) in req.headers() {
            let value = value
                .to_str()
                .expect("Headers must contain only visible ASCII characters")
                .replace("'", r"'\''");

            write!(f, "--header '{}: {}' ", name, value)?;
        }

        if let Some(raw_data) = raw_body {
            write!(f, "--data '{raw_data}'")?;
        }

        write!(f, "'{}'", req.url().to_string().replace("'", "%27"))?;

        Ok(())
    }
}

//impl Postman::Request {
//    fn to_curl(self) -> Result<String, String> {
//
//    }
//}
