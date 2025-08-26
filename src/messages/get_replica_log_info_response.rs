//! GetReplicaLogInfoResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/GetReplicaLogInfoResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Decodable, Decoder,
    Encodable, Encoder, HeaderVersion, Message, StrBytes, VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct GetReplicaLogInfoResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    /// The epoch of the broker.
    ///
    /// Supported API versions: 0
    pub broker_epoch: i64,

    /// True if response does not include all the topic partitions requested. Only the first 1000 topic partitions are returned.
    ///
    /// Supported API versions: 0
    pub has_more_data: bool,

    /// The list of the partition log info.
    ///
    /// Supported API versions: 0
    pub topic_partition_log_info_list: Vec<TopicPartitionLogInfo>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl GetReplicaLogInfoResponse {
    /// Sets `throttle_time_ms` to the passed value.
    ///
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0
    pub fn with_throttle_time_ms(mut self, value: i32) -> Self {
        self.throttle_time_ms = value;
        self
    }
    /// Sets `broker_epoch` to the passed value.
    ///
    /// The epoch of the broker.
    ///
    /// Supported API versions: 0
    pub fn with_broker_epoch(mut self, value: i64) -> Self {
        self.broker_epoch = value;
        self
    }
    /// Sets `has_more_data` to the passed value.
    ///
    /// True if response does not include all the topic partitions requested. Only the first 1000 topic partitions are returned.
    ///
    /// Supported API versions: 0
    pub fn with_has_more_data(mut self, value: bool) -> Self {
        self.has_more_data = value;
        self
    }
    /// Sets `topic_partition_log_info_list` to the passed value.
    ///
    /// The list of the partition log info.
    ///
    /// Supported API versions: 0
    pub fn with_topic_partition_log_info_list(mut self, value: Vec<TopicPartitionLogInfo>) -> Self {
        self.topic_partition_log_info_list = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "broker")]
impl Encodable for GetReplicaLogInfoResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version != 0 {
            bail!("specified version not supported by this message type");
        }
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Int64.encode(buf, &self.broker_epoch)?;
        types::Boolean.encode(buf, &self.has_more_data)?;
        types::CompactArray(types::Struct { version })
            .encode(buf, &self.topic_partition_log_info_list)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Int64.compute_size(&self.broker_epoch)?;
        total_size += types::Boolean.compute_size(&self.has_more_data)?;
        total_size += types::CompactArray(types::Struct { version })
            .compute_size(&self.topic_partition_log_info_list)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

#[cfg(feature = "client")]
impl Decodable for GetReplicaLogInfoResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        if version != 0 {
            bail!("specified version not supported by this message type");
        }
        let throttle_time_ms = types::Int32.decode(buf)?;
        let broker_epoch = types::Int64.decode(buf)?;
        let has_more_data = types::Boolean.decode(buf)?;
        let topic_partition_log_info_list =
            types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            throttle_time_ms,
            broker_epoch,
            has_more_data,
            topic_partition_log_info_list,
            unknown_tagged_fields,
        })
    }
}

impl Default for GetReplicaLogInfoResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            broker_epoch: 0,
            has_more_data: false,
            topic_partition_log_info_list: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for GetReplicaLogInfoResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PartitionLogInfo {
    /// The id for the partition.
    ///
    /// Supported API versions: 0
    pub partition: i32,

    /// The last written leader epoch in the log.
    ///
    /// Supported API versions: 0
    pub last_written_leader_epoch: i32,

    /// The current leader epoch for the partition from the broker point of view.
    ///
    /// Supported API versions: 0
    pub current_leader_epoch: i32,

    /// The log end offset for the partition.
    ///
    /// Supported API versions: 0
    pub log_end_offset: i64,

    /// The result error, or zero if there was no error.
    ///
    /// Supported API versions: 0
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl PartitionLogInfo {
    /// Sets `partition` to the passed value.
    ///
    /// The id for the partition.
    ///
    /// Supported API versions: 0
    pub fn with_partition(mut self, value: i32) -> Self {
        self.partition = value;
        self
    }
    /// Sets `last_written_leader_epoch` to the passed value.
    ///
    /// The last written leader epoch in the log.
    ///
    /// Supported API versions: 0
    pub fn with_last_written_leader_epoch(mut self, value: i32) -> Self {
        self.last_written_leader_epoch = value;
        self
    }
    /// Sets `current_leader_epoch` to the passed value.
    ///
    /// The current leader epoch for the partition from the broker point of view.
    ///
    /// Supported API versions: 0
    pub fn with_current_leader_epoch(mut self, value: i32) -> Self {
        self.current_leader_epoch = value;
        self
    }
    /// Sets `log_end_offset` to the passed value.
    ///
    /// The log end offset for the partition.
    ///
    /// Supported API versions: 0
    pub fn with_log_end_offset(mut self, value: i64) -> Self {
        self.log_end_offset = value;
        self
    }
    /// Sets `error_code` to the passed value.
    ///
    /// The result error, or zero if there was no error.
    ///
    /// Supported API versions: 0
    pub fn with_error_code(mut self, value: i16) -> Self {
        self.error_code = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "broker")]
impl Encodable for PartitionLogInfo {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version != 0 {
            bail!("specified version not supported by this message type");
        }
        types::Int32.encode(buf, &self.partition)?;
        types::Int32.encode(buf, &self.last_written_leader_epoch)?;
        types::Int32.encode(buf, &self.current_leader_epoch)?;
        types::Int64.encode(buf, &self.log_end_offset)?;
        types::Int16.encode(buf, &self.error_code)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition)?;
        total_size += types::Int32.compute_size(&self.last_written_leader_epoch)?;
        total_size += types::Int32.compute_size(&self.current_leader_epoch)?;
        total_size += types::Int64.compute_size(&self.log_end_offset)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

#[cfg(feature = "client")]
impl Decodable for PartitionLogInfo {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        if version != 0 {
            bail!("specified version not supported by this message type");
        }
        let partition = types::Int32.decode(buf)?;
        let last_written_leader_epoch = types::Int32.decode(buf)?;
        let current_leader_epoch = types::Int32.decode(buf)?;
        let log_end_offset = types::Int64.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            partition,
            last_written_leader_epoch,
            current_leader_epoch,
            log_end_offset,
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionLogInfo {
    fn default() -> Self {
        Self {
            partition: 0,
            last_written_leader_epoch: 0,
            current_leader_epoch: 0,
            log_end_offset: 0,
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionLogInfo {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPartitionLogInfo {
    /// The unique topic ID.
    ///
    /// Supported API versions: 0
    pub topic_id: Uuid,

    /// The log info of a partition.
    ///
    /// Supported API versions: 0
    pub partition_log_info: Vec<PartitionLogInfo>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl TopicPartitionLogInfo {
    /// Sets `topic_id` to the passed value.
    ///
    /// The unique topic ID.
    ///
    /// Supported API versions: 0
    pub fn with_topic_id(mut self, value: Uuid) -> Self {
        self.topic_id = value;
        self
    }
    /// Sets `partition_log_info` to the passed value.
    ///
    /// The log info of a partition.
    ///
    /// Supported API versions: 0
    pub fn with_partition_log_info(mut self, value: Vec<PartitionLogInfo>) -> Self {
        self.partition_log_info = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "broker")]
impl Encodable for TopicPartitionLogInfo {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version != 0 {
            bail!("specified version not supported by this message type");
        }
        types::Uuid.encode(buf, &self.topic_id)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.partition_log_info)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Uuid.compute_size(&self.topic_id)?;
        total_size += types::CompactArray(types::Struct { version })
            .compute_size(&self.partition_log_info)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

#[cfg(feature = "client")]
impl Decodable for TopicPartitionLogInfo {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        if version != 0 {
            bail!("specified version not supported by this message type");
        }
        let topic_id = types::Uuid.decode(buf)?;
        let partition_log_info = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            topic_id,
            partition_log_info,
            unknown_tagged_fields,
        })
    }
}

impl Default for TopicPartitionLogInfo {
    fn default() -> Self {
        Self {
            topic_id: Uuid::nil(),
            partition_log_info: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicPartitionLogInfo {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for GetReplicaLogInfoResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}
