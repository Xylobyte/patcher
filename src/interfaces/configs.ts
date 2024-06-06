export interface Project {
    name: string;
    path: string;
    last_opened: string;
}

export interface Config {
    recent_projects: Project[];
}