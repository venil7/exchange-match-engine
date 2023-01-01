import * as t from "io-ts";
import { TxDecoder } from "../decoder/transaction";

export type Tx = t.TypeOf<typeof TxDecoder>;

export type Price = [timesstamp: Date, price: number];

export const toPrice = (tx: Tx): Price => [tx.timestamp, tx.lhs.price];
