import "./RecentProjects.scss"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {Project} from "../../interfaces/configs.ts";
import {EllipsisVertical, FolderSearch} from "lucide-react";
import SmallButton from "../../components/button/SmallButton.tsx";
import Scrollbar from "../../components/scrollbar/Scrollbar.tsx";
import {convertISOToLocalDate} from "../../utils/dates.ts";

export type RecentProjectsProps = {}

function RecentProjects(_props: RecentProjectsProps) {
    const [projects, setProjects] = useState<Project[]>([]);

    useEffect(() => {
        invoke<Project[]>('get_recent_projects')
            .then(p => {
                setProjects(p);

            })
    }, []);

    const sortProjects = projects.sort((a, b) =>
        new Date(b.last_opened).getTime() - new Date(a.last_opened).getTime()
    );

    return <main id="recent-projects" className="flex column align-center justify-center gap30">
        <h1>Liste de vos APIs</h1>
        <section>
            <Scrollbar>
                <ul className="flex column gap10">
                    {sortProjects.map(p => <li className="flex gap15 align-center space-between border-r-small">
                        <div className="flex column gap5">
                            <div className="head flex gap10 align-center">
                                <h3 className="ellipsis">{p.name}</h3>-
                                <span>{convertISOToLocalDate(p.last_opened)}</span>
                            </div>
                            <span className="path">{p.path}</span>
                        </div>

                        <div className="flex gap10">
                            <SmallButton>
                                <FolderSearch/>
                            </SmallButton>

                            <SmallButton>
                                <EllipsisVertical/>
                            </SmallButton>
                        </div>
                    </li>)}
                </ul>
            </Scrollbar>
        </section>
    </main>
}

export default RecentProjects;
