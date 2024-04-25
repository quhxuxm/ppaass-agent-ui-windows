import {AgentServerEventLevel} from "./AgentServerEventLevel.ts";

export class AgentServerEvent {
    constructor(public level: AgentServerEventLevel, public client_address: string, public src_address: string, public dst_address: string, public reason: string) {
    }
}