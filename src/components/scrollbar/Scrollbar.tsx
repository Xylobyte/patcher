import "./Scrollbar.scss"
import React from "react";

export type ScrollbarProps = {
    children: React.ReactNode
}

function Scrollbar(props: ScrollbarProps) {
    return <div className="scrollbar">
        {props.children}
    </div>
}

export default Scrollbar;