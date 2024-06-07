import "./Button.scss"
import React from "react";

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    isPrimary?: boolean;
}

function Button({children, isPrimary, className, ...props}: ButtonProps) {
    return <button
        className={`base-button flex align-center border-r-small ${className} ${isPrimary ? 'primary' : ''}`}
        {...props}>
        {children}
    </button>
}

export default Button;