// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Calculator {
    fn Calculate(&self, p: super::calc::Input) -> ::grpc::result::GrpcResult<super::calc::Output>;
}

pub trait CalculatorAsync {
    fn Calculate(&self, p: super::calc::Input) -> ::grpc::futures_grpc::GrpcFutureSend<super::calc::Output>;
}

// sync client

pub struct CalculatorClient {
    async_client: CalculatorAsyncClient,
}

impl CalculatorClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        CalculatorAsyncClient::new(host, port, tls).map(|c| {
            CalculatorClient {
                async_client: c,
            }
        })
    }
}

impl Calculator for CalculatorClient {
    fn Calculate(&self, p: super::calc::Input) -> ::grpc::result::GrpcResult<super::calc::Output> {
        ::futures::Future::wait(self.async_client.Calculate(p))
    }
}

// async client

pub struct CalculatorAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Calculate: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::calc::Input, super::calc::Output>>,
}

impl CalculatorAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            CalculatorAsyncClient {
                grpc_client: c,
                method_Calculate: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Calculator/Calculate".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl CalculatorAsync for CalculatorAsyncClient {
    fn Calculate(&self, p: super::calc::Input) -> ::grpc::futures_grpc::GrpcFutureSend<super::calc::Output> {
        self.grpc_client.call_unary(p, self.method_Calculate.clone())
    }
}

// sync server

pub struct CalculatorServer {
    async_server: CalculatorAsyncServer,
}

struct CalculatorServerHandlerToAsync {
    handler: ::std::sync::Arc<Calculator + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl CalculatorAsync for CalculatorServerHandlerToAsync {
    fn Calculate(&self, p: super::calc::Input) -> ::grpc::futures_grpc::GrpcFutureSend<super::calc::Output> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Calculate(p)
        })
    }
}

impl CalculatorServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Calculator + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = CalculatorServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        CalculatorServer {
            async_server: CalculatorAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct CalculatorAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl CalculatorAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : CalculatorAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = CalculatorAsyncServer::new_service_def(h);
        CalculatorAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : CalculatorAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Calculator/Calculate".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Calculate(p))
                    },
                ),
            ],
        )
    }
}
