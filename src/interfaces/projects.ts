export interface ApiRequest {
    id: string,
    name: string,
    documentation: string,
    url: string,
    is_folder: boolean,
    children: ApiRequest[],
}

export interface ProjectData {
    api_tree: ApiRequest[]
}

export interface ProjectConfig {
    use_folders_as_url: boolean,
}

export interface ProjectInfo {
    name: string,
    description: string,
    last_edited: string,
    root_url: string,
    config: ProjectConfig
}
