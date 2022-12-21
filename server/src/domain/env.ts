import * as t from "io-ts";
import { EnvDecoder } from "../decoder/env";

export type Env = t.TypeOf<typeof EnvDecoder>;
