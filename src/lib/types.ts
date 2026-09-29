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
export interface EntryCategoryData {
  name: string
  icon?: string | null
  color?: string | null
}
export interface EntryCategoryProps {
  id: string
  name: string
  icon: string | null
  color: string | null
}
export interface EntryNoteData {
  title: string
  content?: string | null
  color?: string | null
  categoryId?: string | null
}
export interface EntryNoteProps extends EntryProps {
  title: string
  color: string | null
}
