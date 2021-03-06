/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/

import { CosmWasmClient, ExecuteResult, SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
export type ExecuteMsg = {
  create_denom: {
    subdenom: string;
    [k: string]: unknown;
  };
};
export interface InstantiateMsg {
  [k: string]: unknown;
}
export type QueryMsg = string;
export interface TokenfactoryReadOnlyInterface {
  contractAddress: string;
}
export class TokenfactoryQueryClient implements TokenfactoryReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
  }

}
export interface TokenfactoryInterface extends TokenfactoryReadOnlyInterface {
  contractAddress: string;
  sender: string;
  createDenom: ({
    subdenom
  }: {
    subdenom: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
}
export class TokenfactoryClient extends TokenfactoryQueryClient implements TokenfactoryInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createDenom = this.createDenom.bind(this);
  }

  createDenom = async ({
    subdenom
  }: {
    subdenom: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_denom: {
        subdenom
      }
    }, fee, memo, funds);
  };
}