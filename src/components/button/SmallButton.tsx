import "./SmallButton.scss"
import React from "react";

export interface SmallButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
}

function SmallButton({children, ...props}: SmallButtonProps) {
    return <button className="small-button grid-center" {...props}>
        {children}
    </button>
}

export default SmallButton;