export interface EntryProps {
  id: string
  createdAt: string
  updatedAt: string
  categoryId: string | null
}
export interface EntryPasswordData {
  url: string
  username: string
  /**
   * `null` means "leave the stored password alone", which is why this is not
   * optional: "no new password" is a value the update takes deliberately, not an
   * absence of one, and `null` says exactly that. `add` narrows it to a `string`
   * with `EntryPasswordData & { password: string }`.
   */
  password: string | null
  categoryId: string | null
}
export interface EntryPasswordProps extends EntryProps {
  url: string
  username: string
}
export interface EntryCategoryData {
  name: string
  icon: string | null
  color: string | null
}
export interface EntryCategoryProps {
  id: string
  name: string
  icon: string | null
  color: string | null
}
export interface EntryNoteData {
  title: string
  color: string | null
  content: string | null
  categoryId: string | null
}
export interface EntryNoteProps extends EntryProps {
  title: string
  color: string | null
}
