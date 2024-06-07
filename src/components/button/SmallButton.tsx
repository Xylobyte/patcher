import "./SmallButton.scss"
import React from "react";

export interface SmallButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
}

function SmallButton({children, className, ...props}: SmallButtonProps) {
    return <button className={`small-button grid-center border-r-small ${className}`} {...props}>
        {children}
    </button>
}

export default SmallButton;