use std::io::{self, Write};
use std::time::Duration;

use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use anyhow::Result;

use dnp3::link::{EndpointAddress, LinkErrorMode};
use dnp3::tcp::{EndpointList, spawn_master_tcp_client};
use dnp3::master::{
    MasterChannelConfig, AssociationConfig, EventClasses, Classes,
    ReadHandler, AssociationHandler, AssociationInformation,
    TimeSyncProcedure,
};
use dnp3::app::{ConnectStrategy, NullListener, Timestamp};

// A ReadHandler that ignores all incoming data.
struct NoopReadHandler;
impl ReadHandler for NoopReadHandler {}

// A custom AssociationHandler that always reports the timestamp we give it.
struct CustomTimeHandler {
    ts: Timestamp,
}
impl AssociationHandler for CustomTimeHandler {
    fn get_current_time(&self) -> Option<Timestamp> {
        Some(self.ts)
    }
}

// A no-op AssociationInformation
struct NoopAssocInfo;
impl AssociationInformation for NoopAssocInfo {}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // 1) Show your PC time and prompt for the desired target time.
    let now = Local::now();
    println!("Local PC time is: {}", now.format("%Y-%m-%d %H:%M:%S"));
    print!("Enter desired date & time (YYYY-MM-DD HH:MM:SS): ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let naive = NaiveDateTime::parse_from_str(buf.trim(), "%Y-%m-%d %H:%M:%S")?;
    // Convert to UTC, then to the dnp3 Timestamp (milliseconds since epoch).
    let dt_utc = Utc
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow::anyhow!("ambiguous or invalid datetime"))?;
    let ts = Timestamp::new(dt_utc.timestamp_millis() as u64);

    // 2) Build and spawn the master channel.
    let master_cfg = MasterChannelConfig::new(EndpointAddress::try_new(0)?);
    let mut channel = spawn_master_tcp_client(
        LinkErrorMode::Close,
        master_cfg,
        EndpointList::new("10.152.152.152:20000".to_string(), &[]),
        ConnectStrategy::default(),
        NullListener::create(),  // <-- explicitly use the null listener
    );

    // 3) Prepare an association config (disable auto-time-sync so we control it manually).
    let mut assoc_cfg = AssociationConfig::new(
        /* disable unsolicited:     */ EventClasses::all(),
        /* enable unsolicited:      */ EventClasses::all(),
        /* startup integrity scan:  */ Classes::all(),
        /* no automatic event scans:*/ EventClasses::none(),
    );
    assoc_cfg.auto_time_sync = None;
    assoc_cfg.keep_alive_timeout = Some(Duration::from_secs(60));

    // 4) Add your outstation (link-address = 10), plugging in your custom time handler.
    let mut association = channel
        .add_association(
            EndpointAddress::try_new(10)?,
            assoc_cfg,
            Box::new(NoopReadHandler),
            Box::new(CustomTimeHandler { ts }),
            Box::new(NoopAssocInfo),
        )
        .await?;

    // 5) Enable the channel (opens the TCP connection).
    channel.enable().await?;

    // 6) Issue the LAN time-sync.
    //    Our CustomTimeHandler::get_current_time will be called to provide the timestamp.
    association
        .synchronize_time(TimeSyncProcedure::Lan)
        .await?;

    println!("Time-sync command sent to outstation 10 (using your supplied timestamp).");
    Ok(())
}
