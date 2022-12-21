import { OrderDecoder, OrderRequestDecoder } from "../decoder/order";
import * as t from "io-ts";
import { v4 as uuidv4 } from "uuid";
import { UUIDBrand } from "io-ts-types";

export enum OrderDirection {
  Buy = "Buy",
  Sell = "Sell",
}

export type Order = t.TypeOf<typeof OrderDecoder>;
export type OrderRequest = t.TypeOf<typeof OrderRequestDecoder>;

export const enrichOrderRequest = (order: Order): OrderRequest => ({
  ...order,
  id: uuidv4() as t.Branded<string, UUIDBrand>,
  timestamp: new Date(),
});
