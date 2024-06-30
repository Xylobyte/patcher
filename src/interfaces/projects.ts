export interface ApiRequest {
    id: string,
    name: string,
    documentation: string,
    url: string,
    is_folder: boolean,
    children: ApiRequest[],
}

export interface ProjectData {
    api_tree: ApiRequest[],
    root_url: string,
}

export interface ProjectConfig {
    use_folder_as_url: boolean,
}

export interface ProjectInfo {
    name: string,
    description: string,
    last_edited: string,
    config: ProjectConfig
}
