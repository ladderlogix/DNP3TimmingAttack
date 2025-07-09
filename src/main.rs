use std::io::{self, Write};
use std::time::Duration;

use anyhow::Result;
use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use clap::Parser;

use dnp3::link::{EndpointAddress, LinkErrorMode};
use dnp3::tcp::{EndpointList, spawn_master_tcp_client};
use dnp3::master::{
    MasterChannelConfig, AssociationConfig, EventClasses, Classes,
    ReadHandler, AssociationHandler, AssociationInformation,
    TimeSyncProcedure,
};
use dnp3::app::{ConnectStrategy, NullListener, Timestamp};

/// Simple CLI for DNP3 time synchronization
#[derive(Parser)]
#[command(name = "dnp3-time-sync", about = "Synchronize time to a DNP3 outstation")]
struct Config {
    /// Outstation IP address and port (default: 10.152.152.152:20000)
    #[arg(short, long, default_value = "10.152.152.152:20000")]
    ip: String,

    /// Desired date & time in 'YYYY-MM-DD HH:MM:SS' format (UTC)
    #[arg(short, long)]
    time: Option<String>,
}

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
    // Parse CLI arguments
    let cfg = Config::parse();

    // Determine timestamp: either from CLI arg or interactive prompt
    let ts = if let Some(ref t) = cfg.time {
        parse_time(t)?
    } else {
        interactive_prompt()?
    };

    // Build and spawn the master channel
    let master_cfg = MasterChannelConfig::new(EndpointAddress::try_new(0)?);
    let mut channel = spawn_master_tcp_client(
        LinkErrorMode::Close,
        master_cfg,
        EndpointList::new(cfg.ip.clone(), &[]),
        ConnectStrategy::default(),
        NullListener::create(),
    );

    // Prepare association config (disable auto time-sync)
    let mut assoc_cfg = AssociationConfig::new(
        EventClasses::all(),
        EventClasses::all(),
        Classes::all(),
        EventClasses::none(),
    );
    assoc_cfg.auto_time_sync = None;
    assoc_cfg.keep_alive_timeout = Some(Duration::from_secs(60));

    // Add the outstation association
    let mut association = channel
        .add_association(
            EndpointAddress::try_new(10)?,
            assoc_cfg,
            Box::new(NoopReadHandler),
            Box::new(CustomTimeHandler { ts }),
            Box::new(NoopAssocInfo),
        )
        .await?;

    // Enable the channel (opens TCP connection)
    channel.enable().await?;

    // Issue the LAN time-sync
    association
        .synchronize_time(TimeSyncProcedure::Lan)
        .await?;

    println!("Time-sync command sent to outstation (using your supplied timestamp).");
    Ok(())
}

/// Parse a time string into a DNP3 Timestamp (milliseconds since epoch)
fn parse_time(s: &str) -> Result<Timestamp> {
    let naive = NaiveDateTime::parse_from_str(s.trim(), "%Y-%m-%d %H:%M:%S")?;
    let dt_utc = Utc
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow::anyhow!("ambiguous or invalid datetime"))?;
    Ok(Timestamp::new(dt_utc.timestamp_millis() as u64))
}

/// Prompt the user interactively for the desired date & time
fn interactive_prompt() -> Result<Timestamp> {
    let now = Local::now();
    println!("Local PC time is: {}", now.format("%Y-%m-%d %H:%M:%S"));
    print!("Enter desired date & time (YYYY-MM-DD HH:MM:SS): ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    parse_time(&buf)
}
