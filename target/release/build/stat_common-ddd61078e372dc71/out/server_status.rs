#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpInfo {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// ip-api,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub continent: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub region_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub city: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub isp: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub org: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub r#as: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub asname: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "11")]
    pub lat: f64,
    #[prost(double, tag = "12")]
    pub lon: f64,
    #[prost(string, tag = "13")]
    pub timezone: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SysInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub os_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub os_arch: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub os_family: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub os_release: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub kernel_version: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8")]
    pub cpu_num: u32,
    #[prost(string, tag = "9")]
    pub cpu_brand: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub cpu_vender_id: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub host_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub latest_ts: u64,
    #[prost(string, tag = "4")]
    pub frame: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub vnstat: bool,
    #[prost(bool, tag = "7")]
    pub online4: bool,
    #[prost(bool, tag = "8")]
    pub online6: bool,
    #[prost(uint64, tag = "9")]
    pub uptime: u64,
    /// load
    #[prost(double, tag = "10")]
    pub load_1: f64,
    #[prost(double, tag = "11")]
    pub load_5: f64,
    #[prost(double, tag = "12")]
    pub load_15: f64,
    ///
    #[prost(double, tag = "13")]
    pub ping_10010: f64,
    #[prost(double, tag = "14")]
    pub ping_189: f64,
    #[prost(double, tag = "15")]
    pub ping_10086: f64,
    #[prost(double, tag = "16")]
    pub time_10010: f64,
    #[prost(double, tag = "17")]
    pub time_189: f64,
    #[prost(double, tag = "18")]
    pub time_10086: f64,
    /// t/u/p/d
    #[prost(uint32, tag = "19")]
    pub tcp: u32,
    #[prost(uint32, tag = "20")]
    pub udp: u32,
    #[prost(uint32, tag = "21")]
    pub process: u32,
    #[prost(uint32, tag = "22")]
    pub thread: u32,
    /// netowrk
    #[prost(uint64, tag = "23")]
    pub network_rx: u64,
    #[prost(uint64, tag = "24")]
    pub network_tx: u64,
    #[prost(uint64, tag = "25")]
    pub network_in: u64,
    #[prost(uint64, tag = "26")]
    pub network_out: u64,
    #[prost(uint64, tag = "27")]
    pub last_network_in: u64,
    #[prost(uint64, tag = "28")]
    pub last_network_out: u64,
    ///
    #[prost(double, tag = "29")]
    pub cpu: f64,
    #[prost(uint64, tag = "30")]
    pub memory_total: u64,
    #[prost(uint64, tag = "31")]
    pub memory_used: u64,
    #[prost(uint64, tag = "32")]
    pub swap_total: u64,
    #[prost(uint64, tag = "33")]
    pub swap_used: u64,
    #[prost(uint64, tag = "34")]
    pub hdd_total: u64,
    #[prost(uint64, tag = "35")]
    pub hdd_used: u64,
    #[prost(string, optional, tag = "36")]
    pub custom: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "37")]
    pub sys_info: ::core::option::Option<SysInfo>,
    #[prost(message, optional, tag = "38")]
    pub ip_info: ::core::option::Option<IpInfo>,
    /// group
    #[prost(string, tag = "39")]
    pub gid: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub alias: ::prost::alloc::string::String,
    #[prost(uint64, tag = "41")]
    pub weight: u64,
    #[prost(string, tag = "42")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "43")]
    pub location: ::prost::alloc::string::String,
    #[prost(bool, tag = "44")]
    pub notify: bool,
    /// false: KiB (1024), true: KB (1000)
    #[prost(bool, tag = "45")]
    pub si: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod server_status_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ServerStatusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServerStatusClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServerStatusClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ServerStatusClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ServerStatusClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn report(
            &mut self,
            request: impl tonic::IntoRequest<super::StatRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/server_status.ServerStatus/Report",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("server_status.ServerStatus", "Report"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod server_status_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ServerStatusServer.
    #[async_trait]
    pub trait ServerStatus: Send + Sync + 'static {
        async fn report(
            &self,
            request: tonic::Request<super::StatRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ServerStatusServer<T: ServerStatus> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServerStatus> ServerStatusServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServerStatusServer<T>
    where
        T: ServerStatus,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/server_status.ServerStatus/Report" => {
                    #[allow(non_camel_case_types)]
                    struct ReportSvc<T: ServerStatus>(pub Arc<T>);
                    impl<T: ServerStatus> tonic::server::UnaryService<super::StatRequest>
                    for ReportSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).report(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReportSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ServerStatus> Clone for ServerStatusServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ServerStatus> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServerStatus> tonic::server::NamedService for ServerStatusServer<T> {
        const NAME: &'static str = "server_status.ServerStatus";
    }
}
