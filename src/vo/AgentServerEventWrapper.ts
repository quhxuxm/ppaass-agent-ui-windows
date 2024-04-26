import {AgentServerEvent} from "./AgentServerEvent.ts";

export class AgentServerEventWrapper {
    constructor(public payload: AgentServerEvent) {
    }
}