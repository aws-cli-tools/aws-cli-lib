use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sts::error::SdkError;
use aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityError;
use aws_sdk_sts::{config::Region, Client};
use aws_types::SdkConfig;
use clap::ValueEnum;
use log::info;
use mockall::*;
use serde_json::json;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputType {
    /// Output as json
    Json,
    /// Output as regular string
    String,
}

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[clap(short, long)]
    region: Option<String>,

    /// Which profile to use.
    #[clap(short, long)]
    profile: Option<String>,
}




pub fn get_region_provider(region: Option<String>) -> RegionProviderChain {
    info!("Getting region details");

    RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"))
}

pub async fn get_aws_config(
    profile: Option<String>,
    region_provider: RegionProviderChain,
) -> SdkConfig {
    if let Some(p) = profile {
        info!("Using profile - {}", p);
        aws_config::from_env()
            .region(region_provider)
            .profile_name(p)
            .load()
            .await
    } else {
        info!("Using default profile");
        aws_config::from_env().region(region_provider).load().await
    }
}
