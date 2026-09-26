export interface EntryProps {
  id: string
  createdAt: string
  updatedAt: string
  categoryId: string | null
}
export interface EntryPasswordData {
  url: string
  username: string
  password?: string | null
  categoryId?: string | null
}
export interface EntryPasswordProps extends EntryProps {
  url: string
  username: string
}
