/// 创建文件请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// 文件元数据
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// 存储节点ID
    #[prost(string, tag = "4")]
    pub node_id: ::prost::alloc::string::String,
    /// 副本节点列表
    #[prost(string, repeated, tag = "5")]
    pub replicas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 复制因子
    #[prost(int32, tag = "6")]
    pub replication_factor: i32,
}
/// 创建文件响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// 文件唯一标识符
    #[prost(string, tag = "2")]
    pub file_id: ::prost::alloc::string::String,
    /// 状态消息
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// 读取文件请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 读取偏移量
    #[prost(int64, tag = "2")]
    pub offset: i64,
    /// 读取长度
    #[prost(int32, tag = "3")]
    pub length: i32,
}
/// 读取文件响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// 文件大小
    #[prost(int64, tag = "2")]
    pub size: i64,
    /// 文件唯一标识符
    #[prost(string, tag = "3")]
    pub file_id: ::prost::alloc::string::String,
}
/// 删除文件请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 是否强制删除
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// 删除文件响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 列出目录请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
    /// 目录路径
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 是否递归列出
    #[prost(bool, tag = "2")]
    pub recursive: bool,
}
/// 列出目录响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    /// 文件信息列表
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<FileInfo>,
    /// 目录信息列表
    #[prost(message, repeated, tag = "2")]
    pub directories: ::prost::alloc::vec::Vec<DirectoryInfo>,
}
/// 文件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub size: i64,
    #[prost(int64, tag = "3")]
    pub created_at: i64,
    #[prost(int64, tag = "4")]
    pub modified_at: i64,
    #[prost(string, tag = "5")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "7")]
    pub replicas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, string", tag = "8")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// 文件校验和
    #[prost(string, tag = "9")]
    pub checksum: ::prost::alloc::string::String,
}
/// 目录信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryInfo {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub created_at: i64,
    #[prost(int64, tag = "3")]
    pub modified_at: i64,
    #[prost(int64, tag = "4")]
    pub file_count: i64,
    #[prost(int64, tag = "5")]
    pub total_size: i64,
}
/// 获取文件属性请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAttributesRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
/// 获取文件属性响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAttributesResponse {
    #[prost(message, optional, tag = "1")]
    pub file_info: ::core::option::Option<FileInfo>,
    #[prost(bool, tag = "2")]
    pub exists: bool,
}
/// 更新文件元数据请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 新的元数据
    #[prost(map = "string, string", tag = "2")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// 更新节点ID
    #[prost(string, tag = "3")]
    pub node_id: ::prost::alloc::string::String,
}
/// 更新文件元数据响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod metadata_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 元数据服务定义
    #[derive(Debug, Clone)]
    pub struct MetadataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MetadataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MetadataServiceClient<T>
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
        ) -> MetadataServiceClient<InterceptedService<T, F>>
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
            MetadataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// 创建文件
        pub async fn create_file(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> Result<tonic::Response<super::CreateResponse>, tonic::Status> {
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
                "/chuangshi.MetadataService/CreateFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 读取文件
        pub async fn read_file(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> Result<tonic::Response<super::ReadResponse>, tonic::Status> {
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
                "/chuangshi.MetadataService/ReadFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 删除文件
        pub async fn delete_file(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
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
                "/chuangshi.MetadataService/DeleteFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 列出目录
        pub async fn list_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status> {
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
                "/chuangshi.MetadataService/ListDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 获取文件属性
        pub async fn get_file_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAttributesRequest>,
        ) -> Result<tonic::Response<super::GetAttributesResponse>, tonic::Status> {
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
                "/chuangshi.MetadataService/GetFileAttributes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 更新文件元数据
        pub async fn update_file_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMetadataRequest>,
        ) -> Result<tonic::Response<super::UpdateMetadataResponse>, tonic::Status> {
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
                "/chuangshi.MetadataService/UpdateFileMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod metadata_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MetadataServiceServer.
    #[async_trait]
    pub trait MetadataService: Send + Sync + 'static {
        /// 创建文件
        async fn create_file(
            &self,
            request: tonic::Request<super::CreateRequest>,
        ) -> Result<tonic::Response<super::CreateResponse>, tonic::Status>;
        /// 读取文件
        async fn read_file(
            &self,
            request: tonic::Request<super::ReadRequest>,
        ) -> Result<tonic::Response<super::ReadResponse>, tonic::Status>;
        /// 删除文件
        async fn delete_file(
            &self,
            request: tonic::Request<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status>;
        /// 列出目录
        async fn list_directory(
            &self,
            request: tonic::Request<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status>;
        /// 获取文件属性
        async fn get_file_attributes(
            &self,
            request: tonic::Request<super::GetAttributesRequest>,
        ) -> Result<tonic::Response<super::GetAttributesResponse>, tonic::Status>;
        /// 更新文件元数据
        async fn update_file_metadata(
            &self,
            request: tonic::Request<super::UpdateMetadataRequest>,
        ) -> Result<tonic::Response<super::UpdateMetadataResponse>, tonic::Status>;
    }
    /// 元数据服务定义
    #[derive(Debug)]
    pub struct MetadataServiceServer<T: MetadataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MetadataService> MetadataServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MetadataServiceServer<T>
    where
        T: MetadataService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/chuangshi.MetadataService/CreateFile" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFileSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::CreateRequest>
                    for CreateFileSvc<T> {
                        type Response = super::CreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chuangshi.MetadataService/ReadFile" => {
                    #[allow(non_camel_case_types)]
                    struct ReadFileSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ReadRequest>
                    for ReadFileSvc<T> {
                        type Response = super::ReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chuangshi.MetadataService/DeleteFile" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFileSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::DeleteRequest>
                    for DeleteFileSvc<T> {
                        type Response = super::DeleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chuangshi.MetadataService/ListDirectory" => {
                    #[allow(non_camel_case_types)]
                    struct ListDirectorySvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ListRequest>
                    for ListDirectorySvc<T> {
                        type Response = super::ListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_directory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDirectorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chuangshi.MetadataService/GetFileAttributes" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileAttributesSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::GetAttributesRequest>
                    for GetFileAttributesSvc<T> {
                        type Response = super::GetAttributesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAttributesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_file_attributes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFileAttributesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chuangshi.MetadataService/UpdateFileMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFileMetadataSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::UpdateMetadataRequest>
                    for UpdateFileMetadataSvc<T> {
                        type Response = super::UpdateMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_file_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateFileMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: MetadataService> Clone for MetadataServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MetadataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MetadataService> tonic::server::NamedService for MetadataServiceServer<T> {
        const NAME: &'static str = "chuangshi.MetadataService";
    }
}
