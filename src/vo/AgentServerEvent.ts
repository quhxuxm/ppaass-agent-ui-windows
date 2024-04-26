import {AgentServerEventType} from "./AgentServerEventType.ts";

export class AgentServerEvent {
    constructor(public content: string, public eventType: AgentServerEventType) {
    }
}