import { TextPlace } from "./text-place";
import { TextSpan } from "./text-span";
import { TextType } from "./text-type";

export interface Diagnostic {
    message: string;
    span: TextSpan;
    place: TextPlace;
    kind: TextType;
}