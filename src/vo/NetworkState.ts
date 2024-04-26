export class NetworkState {
    constructor(public uploadMbAmount: number, public uploadMbPerSecond: number,
                public downloadMbAmount: number, public downloadMbPerSecond: number) {
    }
}