export type RequestMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE"

export type JsonContent = {Json: string}

export interface ApiRequest {
    id: string,
    name: string,
    documentation: string,
    url: string,
    is_folder: boolean,
    method?: RequestMethod,
    body?: JsonContent,
    children?: ApiRequest[],
}

export interface ProjectData {
    api_tree: ApiRequest[]
}

export interface ProjectConfig {
}

export interface ProjectInfo {
    name: string,
    description: string,
    last_edited: string,
    root_url: string,
    config: ProjectConfig
}
