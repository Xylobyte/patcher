import "./Notifications.scss"
import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {CircleCheck, Info, OctagonX} from "lucide-react";

export type NotificationsProps = {}

export type Notification = {
    message: string,
    id: number,
    type: "success" | "error" | "info"
}

export type AddNotification = {
    message: string,
    type: "success" | "error" | "info"
}

export const SEND_NOTIFICATION = "ui:add_notification";

function Notifications(_props: NotificationsProps) {
    const [notifications, setNotifications] = useState<Notification[]>([]);

    useEffect(() => {
        const unsubscribe = listen<AddNotification>(SEND_NOTIFICATION, (e) => {
            const id = Date.now();
            setNotifications(old => [{
                message: e.payload.message,
                id: id,
                type: e.payload.type
            }, ...old]);

            setTimeout(() => {
                setNotifications(old => old.filter(n => n.id !== id));
            }, 4000);
        });

        return () => {
            const unsub = async () => {
                let fn = await unsubscribe
                fn();
            }
            unsub();
        }
    }, []);

    return <div className="notifications-center flex column gap10 align-center">
        {notifications.map(notification =>
            <div key={notification.endTime}
                 className={`notification b-shadow flex align-center gap10 border-r-small ${notification.type}`}>
                {notification.type === "info" && <Info size={16}/>}
                {notification.type === "error" && <OctagonX size={16}/>}
                {notification.type === "success" && <CircleCheck size={16}/>}
                <span>{notification.message}</span>
            </div>
        )}
    </div>
}

export default Notifications;