import "./RecentProjects.scss"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {Project} from "../../interfaces/configs.ts";
import {EllipsisVertical, FolderSearch} from "lucide-react";
import SmallButton from "../../components/button/SmallButton.tsx";
import Scrollbar from "../../components/scrollbar/Scrollbar.tsx";
import {convertISOToLocalDate} from "../../utils/dates.ts";
import {homeDir} from "@tauri-apps/api/path";
import {message} from "@tauri-apps/api/dialog";
import {open} from '@tauri-apps/api/shell';
import ContextMenu, {ContextMenuItem} from "../../components/context-menu/ContextMenu.tsx";
import {writeText} from "@tauri-apps/api/clipboard";
import {emit} from "@tauri-apps/api/event";
import {AddNotification, SEND_NOTIFICATION} from "../../components/notifications/Notifications.tsx";

export type RecentProjectsProps = {}

const contextMenuItems: ContextMenuItem[] = [
    {
        name: "Copier le chemin",
        action: "copyPath"
    },
    {
        name: "Retirer de la liste",
        action: "removeFromList"
    }
];

function RecentProjects(_props: RecentProjectsProps) {
    const [projects, setProjects] = useState<Project[]>([]);
    const [homePath, setHomePath] = useState<string>("null");

    const [contextMenu, setContextMenu] = useState<{
        x: number;
        y: number;
        items: ContextMenuItem[];
        context: Project;
    } | null>(null);

    const getHomePath = async () => {
        const p = await homeDir();
        setHomePath(p);
    }

    const refreshProjects = () => {
        invoke<Project[]>('get_recent_projects')
            .then(p => {
                setProjects(p);
            });
    };

    useEffect(() => {
        getHomePath();
        refreshProjects();

        window.addEventListener('focus', refreshProjects);

        return () => {
            window.removeEventListener('focus', refreshProjects);
        };
    }, []);

    const pathError = (p: string) => {
        message(`The path ${p} doesn't exist!`, {title: `Path not exists`, type: "error"});
    }

    const execAction = async (action: string) => {
        switch (action) {
            case "copyPath":
                if (contextMenu?.context) {
                    await writeText(contextMenu?.context.path);
                    emit(SEND_NOTIFICATION, {
                        message: "Le chemin a bien été copie",
                        type: "info"
                    } as AddNotification);
                }
                break;
            case "removeFromList":
                break;
        }
    }

    const sortProjects = projects.sort((a, b) =>
        new Date(b.last_opened).getTime() - new Date(a.last_opened).getTime()
    );

    return <main id="recent-projects" className="flex column align-center justify-center gap30">
        <h1>Liste de vos APIs</h1>
        <section>
            <Scrollbar>
                <ul className="p-list flex column gap10">
                    {sortProjects.map(p =>
                        <li className="flex gap15 align-center space-between border-r-small" key={p.path}
                            onClick={() => p.path_exists ? "" : pathError(p.path)}>
                            <div className="flex column gap5">
                                <div className="head flex gap10 align-center">
                                    <h3 className="ellipsis">{p.name}</h3>-
                                    <span>{convertISOToLocalDate(p.last_opened)}</span>
                                </div>
                                <span className={`path ellipsis ${p.path_exists ? 'valid' : 'invalid'}`}>
                                    {p.path.startsWith(homePath) ? '~' + p.path.substring(homePath.length - 1) : p.path}
                                </span>
                            </div>

                            <div className="flex gap5">
                                <SmallButton onClick={e => {
                                    e.stopPropagation();
                                    p.path_exists ? open(p.path) : pathError(p.path);
                                }}>
                                    <FolderSearch/>
                                </SmallButton>

                                <SmallButton onClick={e => {
                                    e.stopPropagation();
                                    setContextMenu({
                                        x: e.currentTarget.getBoundingClientRect().left + e.currentTarget.getBoundingClientRect().width / 2,
                                        y: e.currentTarget.getBoundingClientRect().top + e.currentTarget.getBoundingClientRect().height / 2,
                                        items: contextMenuItems,
                                        context: p
                                    });
                                }}>
                                    <EllipsisVertical/>
                                </SmallButton>
                            </div>
                        </li>
                    )}
                </ul>
            </Scrollbar>
        </section>

        {contextMenu && <ContextMenu
            items={contextMenu.items}
            x={contextMenu.x}
            y={contextMenu.y}
            onClose={() => setContextMenu(null)}
            onAction={action => {
                execAction(action);
                setContextMenu(null);
            }}
        />}
    </main>
}

export default RecentProjects;
