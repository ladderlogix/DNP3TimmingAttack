use std::time::Duration;
use anyhow::Result;

// link-layer types
use dnp3::link::{EndpointAddress, LinkErrorMode};

// app-layer helpers
use dnp3::app::{MaybeAsync, ResponseHeader, ConnectStrategy, NullListener};

// master-side core types
use dnp3::master::{
    MasterChannelConfig,
    AssociationConfig,
    EventClasses,
    Classes,
    ReadType,
    ReadHandler,
    AssociationHandler,
    AssociationInformation,
    TimeSyncProcedure,
};

// TCP client helper + endpoint list
use dnp3::tcp::{EndpointList, spawn_master_tcp_client};

/// A ReadHandler that ignores all incoming data.
struct NoopReadHandler;

impl ReadHandler for NoopReadHandler {
    fn begin_fragment(
        &mut self,
        _read_type: ReadType,
        _header: ResponseHeader,
    ) -> MaybeAsync<()> {
        MaybeAsync::ready(())
    }

    fn end_fragment(
        &mut self,
        _read_type: ReadType,
        _header: ResponseHeader,
    ) -> MaybeAsync<()> {
        MaybeAsync::ready(())
    }

    // all other `handle_*` methods come with default no-ops
}

/// A no-op AssociationHandler
struct NoopAssocHandler;
impl AssociationHandler for NoopAssocHandler {}

/// A no-op AssociationInformation
struct NoopAssocInfo;
impl AssociationInformation for NoopAssocInfo {}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // 1) Build the master channel config (link-address = 0)
    let master_cfg = MasterChannelConfig::new(EndpointAddress::try_new(0)?);

    // 2) Spawn the TCP client master channel
    let mut channel = spawn_master_tcp_client(
        LinkErrorMode::Close,
        master_cfg,
        EndpointList::new("10.152.152.152:20000".to_string(), &[]),
        ConnectStrategy::default(),
        NullListener::create(),
    );

    // 3) Prepare an association config (disable auto-time-sync)
    let mut assoc_cfg = AssociationConfig::new(
        /* disable unsolicited:       */ EventClasses::all(),
        /* after integrity, enable:   */ EventClasses::all(),
        /* startup integrity scan:    */ Classes::all(),
        /* no automatic event scans:  */ EventClasses::none(),
    );
    assoc_cfg.auto_time_sync = None;
    assoc_cfg.keep_alive_timeout = Some(Duration::from_secs(60));

    // 4) Add your outstation (link-address = 10)
    let mut association = channel
        .add_association(
            EndpointAddress::try_new(10)?,
            assoc_cfg,
            Box::new(NoopReadHandler),
            Box::new(NoopAssocHandler),
            Box::new(NoopAssocInfo),
        )
        .await?;

    // 5) Enable the channel so it actually opens the TCP connection
    channel.enable().await?;

    // 6) Issue the LAN time-sync
    association
        .synchronize_time(TimeSyncProcedure::Lan)
        .await?;

    println!("Time-sync command sent to outstation 10.");
    Ok(())
}
