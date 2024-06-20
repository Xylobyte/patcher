import {ContextMenuItem} from "../../interfaces/context-menu.ts"

export const contextMenuItems: ContextMenuItem[] = [
    {
        name: "Copier le chemin",
        action: "copyPath"
    },
    {
        name: "Retirer de la liste",
        action: "removeFromList"
    }
]