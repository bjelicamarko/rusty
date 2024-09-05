import { Diagnostic } from "./diagnostic";
import { Pair } from "./pair";

export interface CustomReport {
    diagnostics: Diagnostic[];
    symbol_table: Pair[];
    seconds: number;
}