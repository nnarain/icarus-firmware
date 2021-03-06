//
// wifi.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 26 2022
//

// use esp_idf_svc::eventloop::*;
// use esp_idf_svc::netif::*;
// use esp_idf_svc::nvs::*;

use esp_idf_svc::{
    sysloop::*,
    netif::*,
    nvs::*,
    wifi::*,
};

use embedded_svc::{
    ipv4,
    wifi::*
};

use anyhow::{Result, bail};

use std::{
    sync::Arc,
    time::Duration,
};

/// WiFi Network Stack
pub struct AppWifi {
    // netif_stack: Arc<EspNetifStack>,
    // sys_loop_stack: Arc<EspSysLoopStack>,
    // default_nvs: Arc<EspDefaultNvs>,
    wifi: Box<EspWifi>,
}

impl AppWifi {
    pub fn new() -> Result<Self> {

        let netif_stack = Arc::new(EspNetifStack::new()?);
        let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
        let default_nvs = Arc::new(EspDefaultNvs::new()?);

        let wifi = Box::new(
            EspWifi::new(netif_stack.clone(), sys_loop_stack.clone(), default_nvs.clone())?
        );

        Ok(Self {
            // netif_stack,
            // sys_loop_stack,
            // default_nvs,
            wifi,
        })
    }

    pub fn connect(&mut self, ssid: &str, pass: &str) -> Result<()> {
        let ap_infos = self.wifi.scan()?;
        let ours = ap_infos.into_iter().find(|ap| ap.ssid == ssid);

        let channel = ours.map(|ours| ours.channel);

        let config = ClientConfiguration {
            ssid: ssid.into(),
            password: pass.into(),
            channel,
            ..Default::default()
        };

        self.wifi.set_configuration(&Configuration::Client(config))?;

        Ok(())

    }

    pub fn is_connected(&self) -> anyhow::Result<bool> {
        // self.wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        //     .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

        // let status = self.wifi.get_status();
        let status = self.get_status()?;
        if let Status(ClientStatus::Started(ClientConnectionStatus::Connected(_)), _) = status {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn get_ip_settings(&self) -> anyhow::Result<ipv4::ClientSettings> {
        let status = self.get_status()?;
        if let Status(ClientStatus::Started(ClientConnectionStatus::Connected(ip_status)), _) = status {
            if let ClientIpStatus::Done(ipv4) = ip_status {
                Ok(ipv4)
            }
            else {
                bail!("Not connected")
            }
        }
        else {
            bail!("Not connected")
        }
    }

    pub fn get_status(&self) -> anyhow::Result<Status> {
        self.wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
            .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;
        Ok(self.wifi.get_status())
    }

}
