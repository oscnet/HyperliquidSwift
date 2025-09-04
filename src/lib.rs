use std::collections::HashMap;
use std::sync::Arc;

use hyperliquid_rust_sdk::{
    ExchangeClient, InfoClient, 
    BaseUrl as SdkBaseUrl,
    ClientOrderRequest, ClientOrder, ClientLimit,
    ClientCancelRequest
};
use alloy::signers::local::PrivateKeySigner;
use alloy::primitives::Address;
use thiserror::Error;

uniffi::include_scaffolding!("hyperliquid");

#[derive(Error, Debug)]
pub enum HyperliquidError {
    #[error("Invalid private key: {message}")]
    InvalidPrivateKey { message: String },
    #[error("Network error: {message}")]
    NetworkError { message: String },
    #[error("API error: {message}")]
    ApiError { message: String },
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
}

impl From<hyperliquid_rust_sdk::Error> for HyperliquidError {
    fn from(err: hyperliquid_rust_sdk::Error) -> Self {
        HyperliquidError::ApiError { message: err.to_string() }
    }
}

#[derive(Debug, Clone)]
pub enum BaseUrl {
    Mainnet,
    Testnet,
}

impl From<BaseUrl> for SdkBaseUrl {
    fn from(base_url: BaseUrl) -> Self {
        match base_url {
            BaseUrl::Mainnet => SdkBaseUrl::Mainnet,
            BaseUrl::Testnet => SdkBaseUrl::Testnet,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub asset: String,
    pub is_buy: bool,
    pub size: f64,
    pub price: f64,
    pub reduce_only: bool,
}

#[derive(Debug, Clone)]
pub struct CancelRequest {
    pub asset: String,
    pub oid: u64,
}

#[derive(Debug, Clone)]
pub struct UserState {
    pub address: String,
    pub margin_summary_equity: f64,
    pub margin_summary_account_value: f64,
    pub margin_summary_total_margin_used: f64,
}

#[derive(Debug, Clone)]
pub struct OpenOrder {
    pub asset: String,
    pub is_buy: bool,
    pub size: f64,
    pub price: f64,
    pub oid: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct UserBalance {
    pub token: String,
    pub hold: f64,
    pub total: f64,
}

pub struct HyperliquidExchange {
    client: ExchangeClient,
    runtime: tokio::runtime::Runtime,
    wallet_address: String,
}

impl HyperliquidExchange {
    pub fn new(private_key: String, base_url: BaseUrl) -> Result<Self, HyperliquidError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| HyperliquidError::NetworkError { message: e.to_string() })?;
        
        let wallet = private_key.parse::<PrivateKeySigner>()
            .map_err(|e| HyperliquidError::InvalidPrivateKey { message: e.to_string() })?;
        
        let wallet_address = format!("{:?}", wallet.address());
        
        let client = runtime.block_on(async {
            ExchangeClient::new(None, wallet, Some(base_url.into()), None, None).await
        })?;
        
        Ok(HyperliquidExchange { client, runtime, wallet_address })
    }
    
    pub fn get_wallet_address(&self) -> String {
        self.wallet_address.clone()
    }
    
    pub fn place_order(&self, order: OrderRequest) -> Result<String, HyperliquidError> {
        self.runtime.block_on(async {
            let client_order = ClientOrderRequest {
                asset: order.asset,
                is_buy: order.is_buy,
                reduce_only: order.reduce_only,
                limit_px: order.price,
                sz: order.size,
                order_type: ClientOrder::Limit(ClientLimit {
                    tif: "Gtc".to_string(),
                }),
                cloid: None,
            };
            
            let response = self.client.order(client_order, None).await?;
            Ok(format!("{:?}", response))
        })
    }
    
    pub async fn place_order_async(&self, order: OrderRequest) -> Result<String, HyperliquidError> {
        let client_order = ClientOrderRequest {
            asset: order.asset,
            is_buy: order.is_buy,
            reduce_only: order.reduce_only,
            limit_px: order.price,
            sz: order.size,
            order_type: ClientOrder::Limit(ClientLimit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };
        
        let response = self.client.order(client_order, None).await?;
        Ok(format!("{:?}", response))
    }
    
    pub fn cancel_order(&self, cancel: CancelRequest) -> Result<String, HyperliquidError> {
        self.runtime.block_on(async {
            let cancel_req = ClientCancelRequest {
                asset: cancel.asset,
                oid: cancel.oid,
            };
            
            let response = self.client.cancel(cancel_req, None).await?;
            Ok(format!("{:?}", response))
        })
    }
    
    pub async fn cancel_order_async(&self, cancel: CancelRequest) -> Result<String, HyperliquidError> {
        let cancel_req = ClientCancelRequest {
            asset: cancel.asset,
            oid: cancel.oid,
        };
        
        let response = self.client.cancel(cancel_req, None).await?;
        Ok(format!("{:?}", response))
    }
    
    pub fn cancel_all_orders(&self, asset: Option<String>) -> Result<String, HyperliquidError> {
        self.runtime.block_on(async {
            let response = if let Some(asset_name) = asset {
                let cancel_req = ClientCancelRequest {
                    asset: asset_name,
                    oid: 0, // Cancel all orders for this asset
                };
                self.client.bulk_cancel(vec![cancel_req], None).await?
            } else {
                // Cancel all orders for all assets
                let mut cancel_reqs = Vec::new();
                for asset_name in self.client.coin_to_asset.keys() {
                    cancel_reqs.push(ClientCancelRequest {
                        asset: asset_name.clone(),
                        oid: 0,
                    });
                }
                self.client.bulk_cancel(cancel_reqs, None).await?
            };
            
            Ok(format!("{:?}", response))
        })
    }
    
    pub async fn cancel_all_orders_async(&self, asset: Option<String>) -> Result<String, HyperliquidError> {
        let response = if let Some(asset_name) = asset {
            let cancel_req = ClientCancelRequest {
                asset: asset_name,
                oid: 0, // Cancel all orders for this asset
            };
            self.client.bulk_cancel(vec![cancel_req], None).await?
        } else {
            // Cancel all orders for all assets
            let mut cancel_reqs = Vec::new();
            for asset_name in self.client.coin_to_asset.keys() {
                cancel_reqs.push(ClientCancelRequest {
                    asset: asset_name.clone(),
                    oid: 0,
                });
            }
            self.client.bulk_cancel(cancel_reqs, None).await?
        };
        
        Ok(format!("{:?}", response))
    }
}

pub struct HyperliquidInfo {
    client: InfoClient,
    runtime: tokio::runtime::Runtime,
}

impl HyperliquidInfo {
    pub fn new(base_url: BaseUrl) -> Result<Self, HyperliquidError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| HyperliquidError::NetworkError { message: e.to_string() })?;
        
        let client = runtime.block_on(async {
            InfoClient::new(None, Some(base_url.into())).await
        })?;
        
        Ok(HyperliquidInfo { client, runtime })
    }
    
    pub fn get_user_state(&self, address: String) -> Result<UserState, HyperliquidError> {
        self.runtime.block_on(async {
            let addr = address.parse::<Address>()
                .map_err(|e| HyperliquidError::InvalidInput { message: e.to_string() })?;
            
            let state = self.client.user_state(addr).await?;
            
            Ok(UserState {
                address,
                margin_summary_equity: state.margin_summary.account_value.parse().unwrap_or(0.0),
                margin_summary_account_value: state.margin_summary.account_value.parse().unwrap_or(0.0),
                margin_summary_total_margin_used: state.margin_summary.total_margin_used.parse().unwrap_or(0.0),
            })
        })
    }
    
    pub async fn get_user_state_async(&self, address: String) -> Result<UserState, HyperliquidError> {
        let addr = address.parse::<Address>()
            .map_err(|e| HyperliquidError::InvalidInput { message: e.to_string() })?;
        
        let state = self.client.user_state(addr).await?;
        
        Ok(UserState {
            address,
            margin_summary_equity: state.margin_summary.account_value.parse().unwrap_or(0.0),
            margin_summary_account_value: state.margin_summary.account_value.parse().unwrap_or(0.0),
            margin_summary_total_margin_used: state.margin_summary.total_margin_used.parse().unwrap_or(0.0),
        })
    }
    
    pub fn get_open_orders(&self, address: String) -> Result<Vec<OpenOrder>, HyperliquidError> {
        self.runtime.block_on(async {
            let addr = address.parse::<Address>()
                .map_err(|e| HyperliquidError::InvalidInput { message: e.to_string() })?;
            
            let orders = self.client.open_orders(addr).await?;
            let mut result = Vec::new();
            
            for order in orders {
                result.push(OpenOrder {
                    asset: order.coin,
                    is_buy: order.side == "B", // B for buy, A for sell
                    size: order.sz.parse().unwrap_or(0.0),
                    price: order.limit_px.parse().unwrap_or(0.0),
                    oid: order.oid,
                    timestamp: order.timestamp,
                });
            }
            
            Ok(result)
        })
    }
    
    pub async fn get_open_orders_async(&self, address: String) -> Result<Vec<OpenOrder>, HyperliquidError> {
        let addr = address.parse::<Address>()
            .map_err(|e| HyperliquidError::InvalidInput { message: e.to_string() })?;
        
        let orders = self.client.open_orders(addr).await?;
        let mut result = Vec::new();
        
        for order in orders {
            result.push(OpenOrder {
                asset: order.coin,
                is_buy: order.side == "B", // B for buy, A for sell
                size: order.sz.parse().unwrap_or(0.0),
                price: order.limit_px.parse().unwrap_or(0.0),
                oid: order.oid,
                timestamp: order.timestamp,
            });
        }
        
        Ok(result)
    }
    
    pub fn get_user_balances(&self, address: String) -> Result<Vec<UserBalance>, HyperliquidError> {
        self.runtime.block_on(async {
            let addr = address.parse::<Address>()
                .map_err(|e| HyperliquidError::InvalidInput { message: e.to_string() })?;
            
            let balances = self.client.user_token_balances(addr).await?;
            let mut result = Vec::new();
            
            for balance in balances.balances {
                result.push(UserBalance {
                    token: balance.coin,
                    hold: balance.hold.parse().unwrap_or(0.0),
                    total: balance.total.parse().unwrap_or(0.0),
                });
            }
            
            Ok(result)
        })
    }
    
    pub async fn get_user_balances_async(&self, address: String) -> Result<Vec<UserBalance>, HyperliquidError> {
        let addr = address.parse::<Address>()
            .map_err(|e| HyperliquidError::InvalidInput { message: e.to_string() })?;
        
        let balances = self.client.user_token_balances(addr).await?;
        let mut result = Vec::new();
        
        for balance in balances.balances {
            result.push(UserBalance {
                token: balance.coin,
                hold: balance.hold.parse().unwrap_or(0.0),
                total: balance.total.parse().unwrap_or(0.0),
            });
        }
        
        Ok(result)
    }
    
    pub fn get_all_mids(&self) -> Result<HashMap<String, String>, HyperliquidError> {
        self.runtime.block_on(async {
            let mids = self.client.all_mids().await?;
            Ok(mids)
        })
    }
    
    pub async fn get_all_mids_async(&self) -> Result<HashMap<String, String>, HyperliquidError> {
        let mids = self.client.all_mids().await?;
        Ok(mids)
    }
}

pub fn create_exchange_client(private_key: String, base_url: BaseUrl) -> Result<Arc<HyperliquidExchange>, HyperliquidError> {
    let client = HyperliquidExchange::new(private_key, base_url)?;
    Ok(Arc::new(client))
}

pub fn create_info_client(base_url: BaseUrl) -> Result<Arc<HyperliquidInfo>, HyperliquidError> {
    let client = HyperliquidInfo::new(base_url)?;
    Ok(Arc::new(client))
}