export interface FullModelWithRelationsIds {
    id: string
    server: string
    server_id: string
    profile_id: string
    published: boolean
    title: string
    summary: string
    description: string
    tags: string[]
    license: string
    created_at: string
    updated_at: string
    files: string[]
    images?: string[]
}
