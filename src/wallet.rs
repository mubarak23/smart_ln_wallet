use crate::config::Config::Settings;
use crate::db::Database;
use crate::channel::ChannelManager;
use crate::peer::PeerManager;
use crate::invoice::InvoiceManager;

pub struct LightningWallet {
  pub settings: Settings,
  pub database: Database,
  pub channel_manager: ChannelManager,
  pub peer_manager: PeerManager,
  pub invoice_manager: InvoiceManager
}

impl LightningWallet {
  pub fn new(settings: Settings, database: Database) -> Self {
    let channel_manager = ChannelManager::new(&settings, &database);
    let peer_manager = PeerManager::new(&settings, &channel_manager);
    let invoice_manager = InvoiceManager::new(&settings, &channel_manager);
    
    LightningWallet {
      settings,
      database,
      channel_manager,
      peer_manager,
      invoice_manager
    }
  }
}