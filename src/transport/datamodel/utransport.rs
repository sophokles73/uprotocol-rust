/********************************************************************************
 * Copyright (c) 2023 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

use async_trait::async_trait;

use crate::transport::datamodel::{UAttributes, UPayload, UStatus};
use crate::uri::datamodel::{UEntity, UUri};

use super::UListener;

#[derive(Debug, Clone)]
pub struct UMessage {
    pub topic: UUri,
    pub attributes: UAttributes,
    pub payload: UPayload,
}

#[async_trait]
pub trait UTransport {
    /// API to register the calling uE with the underlying transport implementation.
    async fn register(&self, uentity: UEntity, token: &[u8]) -> Result<(), UStatus>;

    /// Transmit UPayload to the topic using the attributes defined in UTransportAttributes.
    async fn send(
        &self,
        topic: UUri,
        payload: UPayload,
        attributes: UAttributes,
    ) -> Result<(), UStatus>;

    /// Register a method that will be called when a message comes in on the specific topic.
    // async fn register_listener<T>(&self, topic: UUri, listener: T) -> Result<String, UStatus>
    // where
    //     T: Fn(UMessage) + Send + 'static;
    async fn register_listener(&self, topic: UUri, listener: Box<dyn Fn(UMessage) + Send + 'static>) -> Result<String, UStatus>;

    /// Register a method that will be called when a message comes in on the specific topic.
    // async fn register_listener<T>(&self, topic: UUri, listener: T) -> Result<String, UStatus>
    // where
    //     T: Fn(UMessage) + Send + 'static;
    async fn register_ulistener(&self, topic: UUri, listener: Box<dyn UListener + Send + 'static>) -> Result<String, UStatus>;

    /// Unregister a listener. Messages arriving on this topic will no longer be processed by this listener.
    async fn unregister_listener(&self, id: String) -> Result<(), UStatus>;
}
