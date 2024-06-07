import "./ContextMenu.scss"
import React, {useEffect, useRef} from "react";

export type ContextMenuItem = {
    name: string;
    action: string;
}

export type ContextMenuProps = {
    items: ContextMenuItem[];
    x: number;
    y: number;
    onAction: (action: string) => void;
    onClose: () => void;
}

function ContextMenu(props: ContextMenuProps) {
    const menuRef = useRef<HTMLDivElement>(null);

    const handleClickOutside = (event: MouseEvent) => {
        if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
            props.onClose();
        }
    };

    useEffect(() => {
        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);

    const style = {
        top: props.y,
        left: props.x
    } as React.CSSProperties;

    return <div ref={menuRef} className="context-menu b-shadow border-r-small" style={style}>
        <ul className="flex column">
            {props.items.map(item =>
                <li key={item.name} onClick={() => props.onAction(item.action)}>
                    {item.name}
                </li>
            )}
        </ul>
    </div>
}

export default ContextMenu;
