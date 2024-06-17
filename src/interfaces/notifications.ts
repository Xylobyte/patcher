export interface Notification {
    message: string
    id: number
    type: "success" | "error" | "info"
}

export interface AddNotification {
    message: string
    type: "success" | "error" | "info"
}
