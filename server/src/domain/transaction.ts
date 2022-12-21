import * as t from "io-ts";
import { TxDecoder } from "../decoder/transaction";

export type Tx = t.TypeOf<typeof TxDecoder>;
