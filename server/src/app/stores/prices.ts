import { map } from "fp-ts/lib/Either";
import { map as amap } from "fp-ts/lib/Array";
import { pipe } from "fp-ts/lib/function";
import { derived } from "svelte/store";
import { transactions } from "./transactions";
import { Result } from "../../domain/action";

export type Price = [timesstamp: Date, price: number];

export const prices = derived<typeof transactions, Result<Price[]>>(
  transactions,
  ($txs) => {
    return pipe($txs, map(amap((tx) => [tx.timestamp, tx.lhs.price] as Price)));
  }
);
