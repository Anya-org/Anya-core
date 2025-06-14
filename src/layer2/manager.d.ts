// TypeScript definitions for Layer2 Rust modules

export enum Layer2Protocol {
  Lightning = 'Lightning',
  StateChannels = 'StateChannels',
  BOB = 'BOB',
  Liquid = 'Liquid',
  RGB = 'RGB',
  DLC = 'DLC',
  RSK = 'RSK',
  Stacks = 'Stacks',
  TaprootAssets = 'TaprootAssets',
}

export interface Layer2ProtocolTrait {
  get_protocol_type(): Layer2Protocol;
  is_active(): Promise<boolean>;
  get_balance(): Promise<number>;
  transfer(amount: number, recipient: string): Promise<string>;
}

export interface AssetParams {
  asset_id: string;
  amount: number;
  metadata?: Record<string, any>;
}

export interface AssetTransfer {
  from_protocol: Layer2Protocol;
  to_protocol: Layer2Protocol;
  asset_params: AssetParams;
  fee?: number;
}

export interface Proof {
  proof_type: string;
  data: Uint8Array;
  timestamp: number;
}

export class Layer2Manager {
  constructor();
  
  initialize_protocols(): Promise<void>;
  
  get_protocol_status(protocol: Layer2Protocol): Promise<{
    active: boolean;
    balance: number;
    transactions: number;
  }>;
  
  cross_layer_transfer(transfer: AssetTransfer): Promise<{
    success: boolean;
    tx_id: string;
    proof?: Proof;
  }>;
  
  validate_protocol_state(protocol: Layer2Protocol): Promise<boolean>;
}

// Lightning Network specific types
export interface LightningNetwork {
  open_channel(peer_id: string, amount: number): Promise<string>;
  close_channel(channel_id: string): Promise<boolean>;
  send_payment(invoice: string): Promise<string>;
}

// State Channels specific types
export interface StateChannel {
  create_channel(participants: string[], initial_state: any): Promise<string>;
  update_state(channel_id: string, new_state: any): Promise<boolean>;
  close_channel(channel_id: string): Promise<boolean>;
}

// BOB Protocol specific types
export interface BobProtocol {
  bridge_to_bitcoin(amount: number, recipient: string): Promise<string>;
  bridge_from_bitcoin(tx_id: string): Promise<string>;
  get_bridge_status(bridge_id: string): Promise<string>;
}

// Liquid Network specific types
export interface LiquidNetwork {
  peg_in(bitcoin_tx: string): Promise<string>;
  peg_out(amount: number, bitcoin_address: string): Promise<string>;
  issue_asset(amount: number, asset_name: string): Promise<string>;
}

// RGB Protocol specific types
export interface RGBProtocol {
  issue_asset(asset_schema: any, amount: number): Promise<string>;
  transfer_asset(asset_id: string, amount: number, recipient: string): Promise<string>;
  validate_asset(asset_id: string): Promise<boolean>;
}

// DLC specific types
export interface DLCProtocol {
  create_contract(oracle_info: any, outcomes: Record<string, number>): Promise<string>;
  execute_contract(contract_id: string, outcome: string): Promise<string>;
  close_contract(contract_id: string): Promise<boolean>;
}

// RSK specific types
export interface RSKProtocol {
  deploy_contract(bytecode: string, constructor_args: any[]): Promise<string>;
  call_contract(contract_address: string, method: string, args: any[]): Promise<any>;
  bridge_btc(amount: number): Promise<string>;
}

// Stacks specific types
export interface StacksProtocol {
  deploy_contract(contract_code: string): Promise<string>;
  call_contract_function(contract_id: string, function_name: string, args: any[]): Promise<any>;
  transfer_stx(amount: number, recipient: string): Promise<string>;
}

// Taproot Assets specific types
export interface TaprootAssetsProtocol {
  mint_asset(asset_meta: any, amount: number): Promise<string>;
  transfer_asset(asset_id: string, amount: number, recipient: string): Promise<string>;
  burn_asset(asset_id: string, amount: number): Promise<string>;
}
